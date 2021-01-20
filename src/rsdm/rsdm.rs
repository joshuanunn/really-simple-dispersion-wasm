use wasm_bindgen::prelude::*;
use std::f64;
use crate::rsdm::preprocess::MetHour;
use crate::rsdm::disperse::{calc_uz, plume_rise, get_sigma_y, get_sigma_z, wind_components, C};

const AMBIENT_TEMP: f64 = 293.15; // Fixed ambient temperature [K] (20 C)
const BANDS: isize = 10;

/// Source defines the stack (emission source) parameters
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Source {
    pub x: f64,         // Stack location (m)
    pub y: f64,
    pub height: f64,    // Stack height (m)
    pub diameter: f64,  // Stack diameter (m)
    pub velocity: f64,  // Plume velocity at stack tip (m/s)
    pub temp: f64,      // Plume temperature (C)
    pub emission: f64,  // Stack emission rate (g/s)
}

/// RSDM maintains the current state
#[wasm_bindgen]
pub struct RSDM {
    pub source: Source,

    // min, max grid extents and spacing (m)
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
    pub z_min: i32,
    pub z_max: i32,

    pub x_spacing: usize,
    pub y_spacing: usize,
    pub z_spacing: usize,
    pub x_points: usize,
    pub y_points: usize,
    pub z_points: usize,

    // meteorological settings
    pub hours: u32,
    pub wspd: f64,
    pub wdir: f64,
    pub roughness: u8,
    pub pgcat: u8,

    // grid values (absolute and image representation)
    r_grid: Vec<f64>,
    r_disp: Vec<u8>,

    h_grid: Vec<f64>,
    h_disp: Vec<u8>,
}

/// Public methods, exported to JavaScript 
#[wasm_bindgen]
impl RSDM {
    // Create instance of RSDM with default parameters
    pub fn new() -> RSDM {
        // Default emission source
        let source = Source{
            x: 0.0,
            y: 0.0,
            height: 50.0,
            diameter: 0.5,
            velocity: 10.0,
            temp: 60.0,
            emission: 1.0
        };
        
        let mut core = RSDM {
            source: source,
            
            x_min: -2500,
            x_max: 2500,
            y_min: -2500,
            y_max: 2500,
            z_min: 0,
            z_max: 1000,

            x_spacing: 10,
            y_spacing: 10,
            z_spacing: 5,
            x_points: 500,
            y_points: 500,
            z_points: 200,

            hours: 20,
            wspd: 5.0,
            wdir: 235.0, // degrees
            roughness: 0,
            pgcat: b'C',

            r_grid: Vec::new(),
            r_disp: Vec::new(),

            h_grid: Vec::new(),
            h_disp: Vec::new(),
        };

        core.setup_grids();
        core
    }

    /// Change grid resolution
    pub fn set_resolution(&mut self, value: &str) {
        let v_spacing: usize;
        let h_spacing: usize;
        match value {
            "Low" => {h_spacing = 50; v_spacing = 25},
            "Medium" => {h_spacing = 20; v_spacing = 10},
            "High" => {h_spacing = 10; v_spacing = 5},
            _ => panic!(),
        }
        self.x_spacing = h_spacing;
        self.y_spacing = h_spacing;
        self.z_spacing = v_spacing;
        self.setup_grids();
    }

    /// Setup grids
    pub fn setup_grids(&mut self) {
        self.x_points = ((self.x_max - self.x_min) / self.x_spacing as i32) as usize;
        self.y_points = ((self.y_max - self.y_min) / self.y_spacing as i32) as usize;
        self.z_points = ((self.z_max - self.z_min) / self.z_spacing as i32) as usize;
        
        let r_grid_len = self.x_points * self.y_points;
        self.r_grid = vec![0.0; r_grid_len];
        self.r_disp = vec![0; r_grid_len];
        
        let h_grid_len = self.x_points * self.z_points;
        self.h_grid = vec![0.0; h_grid_len];
        self.h_disp = vec![0; h_grid_len];
    }

    pub fn set_elevation(&mut self, value: f64) {
        self.source.height = value;
    }

    pub fn set_diameter(&mut self, value: f64) {
        self.source.diameter = value;
    }

    pub fn set_velocity(&mut self, value: f64) {
        self.source.velocity = value;
    }

    pub fn set_temp(&mut self, value: f64) {
        self.source.temp = value;
    }

    pub fn set_wdir(&mut self, value: f64) {
        self.wdir = value;
    }

    pub fn set_wspd(&mut self, value: f64) {
        self.wspd = value;
    }

    pub fn set_roughness(&mut self, value: &str) {
        match value {
            "urban" => self.roughness = 0,
            "rural" => self.roughness = 1,
            _ => panic!(),
        }
    }

    pub fn set_pgcat(&mut self, value: &str) {
        match value {
            "A" => self.pgcat = b'A',
            "B" => self.pgcat = b'B',
            "C" => self.pgcat = b'C',
            "D" => self.pgcat = b'D',
            "E" => self.pgcat = b'E',
            "F" => self.pgcat = b'F',
            _ => panic!(),
        }
    }

    pub fn width(&self) -> u32 {
        self.x_points as u32
    }
    
    pub fn height(&self) -> u32 {
        self.y_points as u32
    }

    pub fn altitude(&self) -> u32 {
        self.z_points as u32
    }

    pub fn r_grid(&self) -> *const u8 {
       self.r_disp.as_ptr()
    }

    pub fn h_grid(&self) -> *const u8 {
        self.h_disp.as_ptr()
    }

    fn cr_to_linear(&self, col: usize, row: usize) -> usize {
        let row_offset = self.y_points - 1;
        self.x_points * (row_offset - row) + col
    }

    fn ch_to_linear(&self, col: usize, row: usize) -> usize {
        let row_offset = self.z_points - 1;
        self.x_points * (row_offset - row) + col
    }

    pub fn r_grid_max(&self) -> f64 {
        self.r_grid.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    pub fn h_grid_max(&self) -> f64 {
        self.h_grid.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    
    pub fn clear_grids(&mut self) {
        for i in 0..self.r_grid.len() {
            self.r_grid[i] = 0.0;
        }
        for i in 0..self.h_grid.len() {
            self.h_grid[i] = 0.0;
        }
    }

    #[allow(non_snake_case)]
    pub fn iter_disp(&mut self, hours: u32) {

        for _ in 0..hours {
            // Generate an hour of met
            let metline: MetHour;
            if hours > 1 {
                metline = self.gen_met(true);
            } else {
                metline = self.gen_met(false);
            }
            
            // Calculate effective wind speed at stack tip (user specified wind speed is for 10 m)
            let Uz = calc_uz(metline.u, self.source.height, 10.0, metline.pgcat, self.roughness);
            
            // Calculate plume rise using Briggs equations
            let Ts = self.source.temp + 273.15;
            let (dH,Xf) = plume_rise(Uz, self.source.velocity, self.source.diameter, Ts, AMBIENT_TEMP, metline.pgcat);
            let H = self.source.height + dH;
            let Q = self.source.emission;

            let sin_phi = metline.phi.sin();
            let cos_phi = metline.phi.cos();

            // Calculate concentrations for plan view grid (fixed grid height of 0 m)
            for (y,Yr) in (self.y_min..self.y_max).step_by(self.y_spacing).enumerate() {
                for (x,Xr) in (self.x_min..self.x_max).step_by(self.x_spacing).enumerate() {
                    if Uz > 0.5 {
                        let (xx, yy) = wind_components(Xr as f64, Yr as f64, 0.0, 0.0, sin_phi, cos_phi);
                        let xx_corr = xx - (Xf / 1000.0); // Plume rise correction
                        let sig_y = get_sigma_y(metline.pgcat, xx_corr);
                        let sig_z = get_sigma_z(metline.pgcat, xx_corr);
                        
                        let i = self.cr_to_linear(x, y);
                        let conc = C(xx_corr, yy, 0.0, Uz, Q, H, sig_y, sig_z) / hours as f64;
                        self.r_grid[i] += conc;
                    }
                }
            }

            // Calculate concentrations for 2d slice showing height profile along plume
            for (z,Zr) in (self.z_min..self.z_max).step_by(self.z_spacing).enumerate() {
                for (x,Xr) in (self.x_min..self.x_max).step_by(self.x_spacing).enumerate() {
                    if Uz > 0.5 {
                        let xx_corr = (Xr as f64 - Xf) / 1000.0; // Plume rise correction
                        let sig_y = get_sigma_y(metline.pgcat, xx_corr);
                        let sig_z = get_sigma_z(metline.pgcat, xx_corr);
                        
                        let i = self.ch_to_linear(x, z);
                        let conc = C(xx_corr, 0.0, Zr as f64, Uz, Q, H, sig_y, sig_z) / hours as f64;
                        self.h_grid[i] += conc;
                    }
                }
            }
        }
    }
    
    pub fn update_png(&mut self) {
        // Calculate min based on max - use log to band
        let grid_max = self.r_grid_max();
        let min_norm = grid_max.log10().trunc() as isize - BANDS;
        
        // Normalise 2d grid into bands by taking log
        for y in 0..self.y_points {
            for x in 0..self.x_points {
                
                let i = self.cr_to_linear(x, y);
                if self.r_grid[i] > grid_max / 1e10 {
                    let conc_norm = self.r_grid[i].log10().trunc() as isize - min_norm;
                    self.r_disp[i] = conc_norm as u8;
                } else {
                    self.r_disp[i] = 0;
                }
            }
        }

        // Normalise height slice into bands by taking log
        for z in 0..self.z_points {
            for x in 0..self.x_points {
                
                let i = self.ch_to_linear(x, z);
                if self.h_grid[i] > grid_max / 1e10 {
                    let conc_norm = self.h_grid[i].log10().trunc() as isize - min_norm;
                    self.h_disp[i] = conc_norm as u8;
                } else {
                    self.h_disp[i] = 0;
                }
            }
        }
    }
}

const TOLERANCE: f64 = 0.00000001;

fn approx_equal(x :f64, y :f64) -> bool {
    let diff = (x - y).abs();
    diff < TOLERANCE
}

#[test]
fn model_run_test() {
    // Create new *RSDM and populate with fixed values (overwrite defaults)
    let mut dm = RSDM::new();
    
    dm.wspd = 2.0;
    dm.wdir = 130.0;
    dm.source.height = 10.0;
    dm.source.temp = 100.0;
    dm.pgcat = b'A';
    dm.hours = 1;

    dm.x_min = -2500;
    dm.x_max = 2500;
    dm.y_min = -2500;
    dm.y_max =  2500;
    dm.x_spacing = 20;
    dm.y_spacing = 20;
    
    //dm.hCoords = &Grid{xmin: -2500, xmax: 2500, xgap: dm.grid, ymin: 0, ymax: 1000, ygap: dm.grid / 2}
    
    dm.setup_grids();
           
    dm.iter_disp(1);

    //hGridRef := 4.086979994894e-07

    //rGrid := dm.rGrid[23][14]
    
    let grid_ref = 23 * dm.x_points + 14;
    let value = &dm.r_grid[grid_ref];
    let result = approx_equal(*value, 6.366502967443e-08);
    println!("{}", *value);
    assert!(result, *value);

    //hGrid := dm.hGrid[19][181]
    //if !approxEqual(hGrid, hGridRef) {
    //    t.Fatalf("got %v, wanted %v", hGrid, hGridRef)
    //}
}