#![allow(dead_code)]
#![allow(unused_imports)]

mod rsdm;

use crate::rsdm::rsdm::RSDM;
use crate::rsdm::rsdm::Source;
use crate::rsdm::disperse::*;

fn main() {
    let source = Source {
        x: 0.0,
        y: 0.0,
        height: 50.0,
        diameter: 0.5,
        velocity: 10.0,
        temp: 60.0,
        emission: 1.0,
    };

    let mut rsdm = RSDM::new(source);
    rsdm.set_resolution(10);
    
    println!("Hello, world! {}", rsdm.x_spacing);
}
