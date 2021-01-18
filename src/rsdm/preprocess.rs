use wasm_bindgen::prelude::*;
use rand::{thread_rng, Rng};
use crate::rsdm::rsdm::RSDM;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct MetHour {
    pub u: f64,
    pub phi: f64,
    //temp: f64,
    pub pgcat: u8,
}

#[wasm_bindgen]
impl RSDM {
    pub fn gen_met(&self, random: bool) -> MetHour {
        // Generate single hour based on RSDM current values
        if random == false {
            // Convert wind direction from degrees to radians
            let phi = self.wdir * std::f64::consts::PI / 180.0;
            MetHour{u: self.wspd, phi: phi, pgcat: self.pgcat}
        // Generate random set of hours
        } else {
            let mut rng = thread_rng();
            // Generate a random windspeed in range of 1-50 m/s
            let u = rng.gen_range(1.0, 50.0);
            // Generate random wind direction 0 - 359 degrees and convert to radians
            let phi = (rng.gen_range(0, 360) as f64) * std::f64::consts::PI / 180.0;
            // Select random PG class
            let pgcat: u8 = rng.gen_range(65, 71); // ASCII A to F
            
            MetHour{u, phi, pgcat}
        }
    }
}