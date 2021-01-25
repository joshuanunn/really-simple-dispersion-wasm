# Really Simple Dispersion (Wasm)

## Introduction
An interactive atmospheric dispersion model for simulation and visualisation of industrial plant emissions. Written in Rust, compiled to WebAssembly (Wasm) using wasm-pack, runs fully in the browser.

**LIVE EXAMPLE** served from GitHub repo: [https://joshuanunn.co.uk/really-simple-dispersion-wasm/example](https://joshuanunn.co.uk/really-simple-dispersion-wasm/example)

In the live example above, the properties of the industrial source and the weather conditions can be tweaked to see the effect on dispersion. Each change will plot average concentrations for a single hour. The *Simulate* button can be used to run a simulation for the specified number of hours of random weather conditions, which builds up a picture of how concentrations may look time-averaged over a longer period with varied conditions.

## Comparison to pure JavaScript version

This project has also been implemented fully in JavaScript in order to compare performance. The corresponding JS code and live demo can be found at [https://github.com/joshuanunn/really-simple-dispersion](https://github.com/joshuanunn/really-simple-dispersion).

## Background

This is Gaussian based dispersion model, which is a mathematical simulation of how air pollution disperses in the atmosphere. The form of the plume exiting a stack at a given height is derived from the overall Gaussian plume equation below, which describes the pollutant concentration at a single position (x,y,z) in a plume referenced coordinate system. The sigma plume formulas σ<sub>y</sub> and σ<sub>z</sub> are power-law or logarithmic functions which vary with meteorological conditions. Briggs plume rise formulas are used to add an additional plume rise due to the buoyancy or momentum of the plume. A typical schematic of a plume is shown below (centre).

![Gaussian Equation](./gaussian_equation.gif)

The plume is visualised by mapping concentrations onto a regular gridded areas, which are effectively 2D slices through a 3D plume. Typical outputs showing simulated ground level emissions over a 5km x 5km area centred on an industrial facility time-averaged over 20 hours is shown below (left). The corresponding side elevation profile of the plume average cross-section with height up to 1km altitude (right). These are contoured on a log scale (each contour is 10x lower/higher than the next).

![Gaussian Views Image](./gaussian_view.png)

NOTE that while this simulation can help with understanding and visualising how Gaussian plume dispersion models work, the ISC-3 model on which it is based has long-since been replaced by modern atmospheric dispersion models such as AERMOD (US) and ADMS (UK).

## Technical Details and Wasm Compilation
Most of the core functionality and program state is encapsulated in an RSDM struct. The concentration maps are held in two internal <f64> vectors for each plot - one to hold running raw concentrations and another to build a contour map. The default "High" image quality is based on a 500 x 500 element array for the plan view (5km @ 10m pixel spacing) and a 500 x 200 element array for the height profile (5 km @ 10m horizontal spacing and 1km @ 5m vertical spacing). These are converted to png images and returned to the browser on each call.

To compile any changes, run the following from the base project directory:
```sh
$ wasm-pack build --target web
$ mv pkg/really_simple_dispersion_wasm.js example/
$ mv pkg/really_simple_dispersion_wasm_bg.wasm example/
```

If serving locally, make sure that the server can support the mimetype 'application/wasm' for .wasm files.

## Tests
Unit tests can then be run by executing the following from the base project directory:
```sh
$ cargo test
```

## License
This software is released under the MIT license [MIT](LICENSE).

This software is based upon equations derived from the Industrial Source Complex (ISC-3) atmospheric dispersion model developed by the US EPA. More details can be found in the ISC-3 user guide: [http://www.epa.gov/scram001/userg/regmod/isc3v2.pdf](http://www.epa.gov/scram001/userg/regmod/isc3v2.pdf).
