
// Source defines the stack (emission source) parameters
pub struct Source {
    pub x: f64,         // Stack location (m)
    pub y: f64,
	pub height: f64,    // Stack height (m)
	pub diameter: f64,  // Stack diameter (m)
	pub velocity: f64,  // Plume velocity at stack tip (m/s)
	pub temp: f64,      // Plume temperature (C)
	pub emission: f64,  // Stack emission rate (g/s)
}

/// RSDM contains all key data variables for maintaining state
pub struct RSDM {
	pub source: Source,

    // min, max grid extents and spacing (m)
	pub x_extents: (isize, isize),
    pub y_extents: (isize, isize),
    pub x_spacing: usize,
    pub y_spacing: usize,

    // grid values (absolute and image representation)
    pub r_grid: Vec<u8>,
	pub r_disp: Vec<u8>,

	//hCoords: Grid,
	//hGrid: Vec<u8>,
	//hDisp: Vec<u8>,
}

impl RSDM {
    pub fn new(source: Source) -> RSDM {
        RSDM {
            source: source,
            
            x_extents: (-2500, 2500),
            y_extents: (-2500, 2500),
            x_spacing: 20,
            y_spacing: 20,

            r_grid: Vec::new(),
            r_disp: Vec::new(),
        }
    }

    pub fn set_resolution(&mut self, spacing: usize) {
        self.x_spacing = spacing;
        self.y_spacing = spacing;
    }
}