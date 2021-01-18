use wasm_bindgen::prelude::*;
use std::f64;
use crate::rsdm::preprocess::MetHour;
use crate::rsdm::disperse::{calc_uz, plume_rise, AMBIENT_TEMP, get_sigma_y, get_sigma_z, wind_components, C};

const BANDS: isize = 10;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    //#[wasm_bindgen(js_namespace = console)]
    //fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    //#[wasm_bindgen(js_namespace = console, js_name = log)]
    //fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64);
}

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
    pub x_spacing: usize,
    pub y_spacing: usize,
    pub x_points: usize,
    pub y_points: usize,

    // meteorological settings
    pub hours: u32,
    pub wspd: f64,
    pub wdir: f64,
    pub roughness: u8,
    pub pgcat: u8,

    // grid values (absolute and image representation)
    r_grid: Vec<f64>,
	r_disp: Vec<u8>,

	//h_grid: Vec<f64>,
	//h_disp: Vec<u8>,
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
            x_spacing: 20,
            y_spacing: 20,
            x_points: 250,
            y_points: 250,

            hours: 20,
            wspd: 5.0,
            wdir: 235.0, // degrees
            roughness: 0,
            pgcat: b'C',

            r_grid: Vec::new(),
            r_disp: Vec::new(),
        };

        core.setup_grids();
        core
    }

    /// Change grid resolution
    pub fn set_resolution(&mut self, spacing: usize) {
        self.x_spacing = spacing;
        self.y_spacing = spacing;
    }

    /// Setup grids
    pub fn setup_grids(&mut self) {
        self.x_points = ((self.x_max - self.x_min) / self.x_spacing as i32) as usize;
        self.y_points = ((self.y_max - self.y_min) / self.y_spacing as i32) as usize;
        let grid_len = self.x_points * self.y_points;
        self.r_grid = vec![0.0; grid_len];
        self.r_disp = vec![0; grid_len];
    }

    pub fn width(&self) -> usize {
        self.x_points
    }
    
    pub fn height(&self) -> usize {
        self.y_points
    }

    pub fn grid(&self) -> *const u8 {
       self.r_disp.as_ptr()
    }

    pub fn cr_to_linear(&self, col: usize, row: usize) -> usize {
        let row_offset = self.y_points - 1;
        self.x_points * (row_offset - row) + col
    }

    pub fn grid_min(&self) -> f64 {
        self.r_grid.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    pub fn grid_max(&self) -> f64 {
        self.r_grid.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

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
        }
    }
    
    pub fn update_png(&mut self) {
        // Calculate min based on max - use log to band
        let grid_max = self.grid_max();
        let min_norm = grid_max.log10().trunc() as isize - BANDS;
        
        // Normalise 2d grid into bands by taking log
        for (y,Yr) in (self.y_min..self.y_max).step_by(self.y_spacing).enumerate() {
            for (x,Xr) in (self.x_min..self.x_max).step_by(self.x_spacing).enumerate() {
                
                let i = self.cr_to_linear(x, y);
                if self.r_grid[i] > grid_max / 1e10 {
                    // 
                    let conc_norm = self.r_grid[i].log10().trunc() as isize - min_norm;
                    self.r_disp[i] = conc_norm as u8;
                } else {
                    self.r_disp[i] = 0;
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
	//	t.Fatalf("got %v, wanted %v", hGrid, hGridRef)
	//}
}