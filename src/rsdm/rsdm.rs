use wasm_bindgen::prelude::*;

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
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    x_spacing: u32,
    y_spacing: u32,

    // grid values (absolute and image representation)
    r_grid: Vec<u8>,
	r_disp: Vec<u8>,

	//hCoords: Grid,
	//hGrid: Vec<u8>,
	//hDisp: Vec<u8>,
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
        
        RSDM {
            source: source,
            
            x_min: -2500,
            x_max: 2500,
            y_min: -2500,
            y_max: 2500,
            x_spacing: 20,
            y_spacing: 20,

            r_grid: Vec::new(),
            r_disp: Vec::new(),
        }
    }

    /// Change grid resolution
    pub fn set_resolution(&mut self, spacing: u32) {
        self.x_spacing = spacing;
        self.y_spacing = spacing;
    }
}
