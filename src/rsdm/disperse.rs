use crate::rsdm::rsdm::{TOLERANCE, approx_equal};

#[allow(non_upper_case_globals)]
const g :f64 = 9.80616; // Gravitational constant
const URBAN: u8 = 0;
const RURAL: u8 = 1;

fn sigma_y(c: f64, d: f64, x: f64) -> f64 {
    let theta: f64 = 0.017453293 * (c - d*x.ln());
    465.11628 * x * theta.tan()
}

pub fn get_sigma_y(pgcat: u8, x: f64) -> f64 {
    match pgcat {
        b'A' => sigma_y(24.1670, 2.5334, x),
        b'B' => sigma_y(18.3330, 1.8096, x),
        b'C' => sigma_y(12.5000, 1.0857, x),
        b'D' => sigma_y(8.3330, 0.72382, x),
        b'E' => sigma_y(6.2500, 0.54287, x),
        b'F' => sigma_y(4.1667, 0.36191, x),
        _ => panic!(),
    }
}

fn sigma_z(a: f64, b: f64, x: f64) -> f64 {
    a * x.powf(b)
}

pub fn get_sigma_z(pgcat: u8, x: f64) -> f64 {
    match pgcat {
        b'A' => sigma_za(x),
        b'B' => sigma_zb(x),
        b'C' => sigma_zc(x),
        b'D' => sigma_zd(x),
        b'E' => sigma_ze(x),
        b'F' => sigma_zf(x),
        _ => panic!(),
    }
}

fn sigma_za(x: f64) -> f64 {
    let s_z = match x {
        sz if sz <= 0.10 => sigma_z(122.800, 0.94470, x),
        sz if sz <= 0.15 => sigma_z(158.080, 1.05420, x),
        sz if sz <= 0.20 => sigma_z(170.220, 1.09320, x),
        sz if sz <= 0.25 => sigma_z(179.520, 1.12620, x),
        sz if sz <= 0.30 => sigma_z(217.410, 1.26440, x),
        sz if sz <= 0.40 => sigma_z(258.890, 1.40940, x),
        sz if sz <= 0.50 => sigma_z(346.750, 1.72830, x),
        sz if sz <= 3.11 => sigma_z(453.850, 2.11660, x),
        _ => 5000.0,
    };
    if s_z > 5000.0 {
        return 5000.0;
    }
    s_z
}

fn sigma_zb(x: f64) -> f64 {
    let s_z = match x {
        sz if sz <= 0.20 => sigma_z(90.673, 0.93198, x),
        sz if sz <= 0.40 => sigma_z(98.483, 0.98332, x),
        _ => sigma_z(109.300, 1.09710, x),
    };
    if s_z > 5000.0 {
        return 5000.0
    }
    s_z
}

fn sigma_zc(x: f64) -> f64 {
    let s_z = sigma_z(61.141, 0.91465, x);
    if s_z > 5000.0 {
        return 5000.0;
    }
    s_z
}

fn sigma_zd(x: f64) -> f64 {
    let s_z = match x {
        sz if sz <= 0.30 => sigma_z(34.459, 0.86974, x),
        sz if sz <= 1.0 => sigma_z(32.093, 0.81066, x),
        sz if sz <= 3.0 => sigma_z(32.093, 0.64403, x),
        sz if sz <= 10.0 => sigma_z(33.504, 0.60486, x),
        sz if sz <= 30.0 => sigma_z(36.650, 0.56589, x),
        _ => sigma_z(44.053, 0.51179, x),
    };
    s_z
}

fn sigma_ze(x: f64) -> f64 {
    let s_z = match x {
        sz if sz <= 0.10 => sigma_z(24.260, 0.83660, x),
        sz if sz <= 0.3 => sigma_z(23.331, 0.81956, x),
        sz if sz <= 1.0 => sigma_z(21.628, 0.75660, x),
        sz if sz <= 2.0 => sigma_z(21.628, 0.63077, x),
        sz if sz <= 4.0 => sigma_z(22.534, 0.57154, x),
        sz if sz <= 10.0 => sigma_z(24.703, 0.50527, x),
        sz if sz <= 20.0 => sigma_z(26.970, 0.46713, x),
        sz if sz <= 40.0 => sigma_z(35.420, 0.37615, x),
        _ => sigma_z(47.618, 0.29592, x),
    };
    s_z
}

fn sigma_zf(x: f64) -> f64 {
    let s_z = match x {
        sz if sz <= 0.20 => sigma_z(15.209, 0.81558, x),
        sz if sz <= 0.7 => sigma_z(14.457, 0.78407, x),
        sz if sz <= 1.0 => sigma_z(13.953, 0.68465, x),
        sz if sz <= 2.0 => sigma_z(13.953, 0.63227, x),
        sz if sz <= 3.0 => sigma_z(14.823, 0.54503, x),
        sz if sz <= 7.0 => sigma_z(16.187, 0.46490, x),
        sz if sz <= 15.0 => sigma_z(17.836, 0.41507, x),
        sz if sz <= 30.0 => sigma_z(22.651, 0.32681, x),
        sz if sz <= 60.0 => sigma_z(27.074, 0.27436, x),
        _ => sigma_z(34.219, 0.21716, x),
    };
    s_z
}

/*
Calculate effective wind speed, using "power law" method
Uz          [m/s]   estimated wind speed at target elevation z
z           [m]     target elevation
uzref       [m/s]   wind speed of actual measurment
zref        [m]     elevation of actual measurement
p           []      wind profile exponent factor (based on stability cat)
*/
pub fn calc_uz(uz_ref: f64, z: f64, z_ref: f64, pgcat: u8, roughness: u8) -> f64 {
    let p: f64 = match roughness {
        URBAN => match pgcat {
            b'A' => 0.15,
            b'B' => 0.15,
            b'C' => 0.20,
            b'D' => 0.25,
            b'E' => 0.30,
            b'F' => 0.30,
            _ => panic!(),
        },
        RURAL => match pgcat {
            b'A' => 0.07,
            b'B' => 0.07,
            b'C' => 0.10,
            b'D' => 0.15,
            b'E' => 0.35,
            b'F' => 0.55,
            _ => panic!(),
        },
        _ => panic!(),
    };
    uz_ref * (z/z_ref).powf(p)
}

/*
Calculate the downwind (x) and crosswind (y) plume components from rectangular coordinates.

inputs:
e_r         [m]     receptor easting
n_r         [m]     receptor northing
e_s         [m]     source (stack) easting
n_s         [m]     source (stack) northing
sin_phi     []      sine of wind direction in radians
cos_phi     []      cosine of wind direction in radians

returns:
x           [km]    downwind plume receptor distance
y           [m]     crosswind plume receptor distance
*/
pub fn wind_components(e_r: f64, n_r: f64, e_s: f64, n_s: f64, sin_phi: f64, cos_phi: f64) -> (f64, f64) {
    let x = (-1.0*(e_r-e_s)*sin_phi - (n_r-n_s)*cos_phi) / 1000.0;
    let y = (e_r-e_s)*cos_phi - (n_r-n_s)*sin_phi;
    return (x, y)
}

/*
Calculate plume rise, using Briggs model.

Calculates the plume rise (dH) to add to stack height and a downwind plume offset (Xf).
us          [m/s]   wind velocity at stack tip
vs          [m/s]   stack exit velocity
ds          [m]     stack tip diameter
Ts          [K]     stack tip temperature
Ta          [K]     ambient temperature
pgcat       []      Pasquill-Gifford stability category

dH          [m]     plume rise
Xf          [m]     plume rise offset
*/
#[allow(non_snake_case)]
pub fn plume_rise(us: f64, vs: f64, ds: f64, Ts: f64, Ta: f64, pgcat: u8) -> (f64, f64) {
    // Compute buoyancy flux
    let Fb = g * vs * ds * ds * (Ts - Ta) / (4.0 * Ts);
    // Calculate momentum flux
    let Fm = vs * vs * ds * ds * Ta / (4.0 * Ts);

    let Xf;
    let dH;

    // Stable PG categories
    if pgcat == b'E' || pgcat == b'F' {
        let eta;
        if pgcat == b'E' {
            eta = 0.020;
        } else {
            eta = 0.035;
        }
        let s = g * eta / Ta;
        let dT = 0.019582 * Ts * vs * s.sqrt();
        // Buoyancy dominated
        if (Ts - Ta) >= dT {
            Xf = 2.0715 * us / s.sqrt();
            dH = 2.6 * (Fb/(us*s)).powf(0.333333333333);
            // Momentum dominated
        } else {
            Xf = 0.0;
            // Calculate unstable/neutral and stable plume rise and take min
            let prUN = 3.0 * ds * vs / us;
            let prS = 1.5 * (Fm/(us*s.sqrt()).powf(0.333333333333));
            dH = prUN.min(prS);
        }
    // Unstable or neutral PG categories
    } else {
        // Unstable or neutral
        if Fb < 55.0 {
            // Check for buoyancy dominated or momentum
            let dT = 0.0297 * Ts * vs.powf(0.333333333333) / ds.powf(0.666666666667);
            // Buoyancy dominated
            if (Ts - Ta) >= dT {
                Xf = 49.0 * Fb.powf(0.625);
                dH = 21.425 * Fb.powf(0.75) / us;
                // Momentum dominated
            } else {
                Xf = 0.0;
                dH = 3.0 * ds * vs / us;
            }
        } else {
            let dT = 0.00575 * Ts * vs.powf(0.666666666667) / ds.powf(0.333333333333);
            if (Ts - Ta) >= dT {
                Xf = 119.0 * Fb.powf(0.4);
                dH = 38.71 * Fb.powf(0.6) / us;
            } else {
                Xf = 0.0;
                dH = 3.0 * ds * vs / us;
            }
        }
    }
    (dH, Xf)
}

/*
Calculate concentration at distance x along plume, at perpendicular offset y and height z
x           [km]    receptor distance downwind along plume centreline
y           [m]     receptor perpendicular offset from plume centreline
z           [m]     receptor height
u_z         [m/s]   wind speed at stack exit
conc        [g/m3]  calculated receptor concentration
*/
#[allow(non_snake_case)]
pub fn C(x: f64, y: f64, z: f64, u_z: f64, Q: f64, H: f64, s_y: f64, s_z: f64) -> f64 {
    // Early return if coordinate upwind, as concentration always zero
    if x <= 0.0 {
        return 0.0;
    }

    let c1 = Q / (2.0 * std::f64::consts::PI * u_z * s_y * s_z);
    let c2 = std::f64::consts::E.powf(-1.0 * (z - H) * (z - H) / (2.0 * s_z * s_z));
    let c3 = std::f64::consts::E.powf(-1.0 * (z + H) * (z + H) / (2.0 * s_z * s_z));
    let c4 = std::f64::consts::E.powf(-1.0 * y * y / (2.0 * s_y * s_y));

    let conc = c1 * (c2 + c3) * c4; // g/m3
    if conc.is_nan() {
        return 0.0;
    }
    conc
}

#[test]
#[allow(non_snake_case)]
fn plume_rise_test() {
    // Example from:
    // https://ceprofs.civil.tamu.edu/qying/cven301_fall2014_arch/lecture7_c.pdf
    let vs = 20.0;  // m/s
    let ds = 5.0;   // m
    let U = 6.0;    // m/s
    let Ts = 400.0; // K
    let Ta = 280.0; // K
    let pgcat = b'D';

    let (dH, Xf) = plume_rise(U, vs, ds, Ts, Ta, pgcat);
    
    assert!(approx_equal(dH, 223.352113600373), "error");
    assert!(approx_equal(Xf, 1264.034881130080), "error");
}

#[test]
fn sigma_y_test() {
    // stability class D, 0.5km downwind, example from:
    // http://faculty.washington.edu/markbenj/CEE357/CEE%20357%20air%20dispersion%20models.pdf
    assert!(approx_equal(get_sigma_y(b'D', 0.5), 36.146193496038), "error");

    // stability class A, 0.997km downwind
    assert!(approx_equal(get_sigma_y(b'A', 0.997), 208.157523627706), "error");

    // stability class B, 12.345m downwind
    assert!(approx_equal(get_sigma_y(b'B', 0.012345), 2.835970876943), "error");

    // stability class C, 27.85km downwind
    assert!(approx_equal(get_sigma_y(b'C', 27.85), 2025.696103458910), "error");

    // stability class D, 5.78m upwind
    assert!(get_sigma_y(b'D', -0.00578).is_nan(), "error");

    // stability class E, 445m downwind
    assert!(approx_equal(get_sigma_y(b'E', 0.445), 24.275915684479), "error");

    // stability class F, 7.5558km downwind
    assert!(approx_equal(get_sigma_y(b'F', 7.5558), 210.931775211803), "error");
}

#[test]
fn sigma_z_test() {
    // stability class D, 0.5km downwind, example from:
    // http://faculty.washington.edu/markbenj/CEE357/CEE%20357%20air%20dispersion%20models.pdf
    assert!(approx_equal(get_sigma_z(b'D', 0.5), 18.296892641654), "error");
    
    // stability class D, 5.78m upwind
    assert!(get_sigma_z(b'D', -0.00578).is_nan(), "error");
    
    // stability class A, 50m downwind
    assert!(approx_equal(get_sigma_z(b'A', 0.05), 7.246283645973), "error");
    
    // stability class A, 270m downwind
    assert!(approx_equal(get_sigma_z(b'A', 0.27), 41.523682287423), "error");
    
    // stability class A, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'A', 2.86), 4196.204889704382), "error");
    
    // stability class A, 54km downwind
    assert!(approx_equal(get_sigma_z(b'A', 54.0), 5000.0), "error");
    
    // stability class B, 50m downwind
    assert!(approx_equal(get_sigma_z(b'B', 0.05), 5.558326444834), "error");
    
    // stability class B, 270m downwind
    assert!(approx_equal(get_sigma_z(b'B', 0.27), 27.177523893054), "error");
    
    // stability class B, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'B', 2.86), 346.177898273921), "error");
    
    // stability class B, 54km downwind
    assert!(approx_equal(get_sigma_z(b'B', 54.0), 5000.0), "error");
    
    // stability class C, 50m downwind
    assert!(approx_equal(get_sigma_z(b'C', 0.05), 3.947711911749), "error");
    
    // stability class C, 270m downwind
    assert!(approx_equal(get_sigma_z(b'C', 0.27), 18.459902569036), "error");
    
    // stability class C, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'C', 2.86), 159.862915743170), "error");
    
    // stability class C, 54km downwind
    assert!(approx_equal(get_sigma_z(b'C', 54.0), 2348.910612301645), "error");
    
    // stability class D, 50m downwind
    assert!(approx_equal(get_sigma_z(b'D', 0.05), 2.545334368597), "error");
    
    // stability class D, 270m downwind
    assert!(approx_equal(get_sigma_z(b'D', 0.27), 11.034101898944), "error");
    
    // stability class D, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'D', 2.86), 63.142784897226), "error");
    
    // stability class D, 54km downwind
    assert!(approx_equal(get_sigma_z(b'D', 54.0), 339.310493995667), "error");
    
    // stability class E, 50m downwind
    assert!(approx_equal(get_sigma_z(b'E', 0.05), 1.979015073784), "error");
    
    // stability class E, 270m downwind
    assert!(approx_equal(get_sigma_z(b'E', 0.27), 7.978143439122), "error");
    
    // stability class E, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'E', 2.86), 41.083717338729), "error");
    
    // stability class E, 54km downwind
    assert!(approx_equal(get_sigma_z(b'E', 54.0), 155.031915174584), "error");
    
    // stability class F, 50m downwind
    assert!(approx_equal(get_sigma_z(b'F', 0.05), 1.321315762922), "error");
    
    // stability class F, 270m downwind
    assert!(approx_equal(get_sigma_z(b'F', 0.27), 5.178781257565), "error");
    
    // stability class F, 2.86km downwind
    assert!(approx_equal(get_sigma_z(b'F', 2.86), 26.282658227590), "error");
    
    // stability class F, 54km downwind
    assert!(approx_equal(get_sigma_z(b'F', 54.0), 80.882017663045), "error");
}

#[test]
fn calc_uz_test() {
    let u_case1 = calc_uz(3.5, 100.0, 10.0, b'D', RURAL);
    assert!(approx_equal(u_case1, 4.943881406180), "error");
    
    let u_case2 = calc_uz(10.0, 50.0, 45.0, b'A', URBAN);
    assert!(approx_equal(u_case2, 10.159296222811), "error");
}

#[test]
#[allow(non_snake_case)]
fn C_test() {
    // Example from:
    // http://faculty.washington.edu/markbenj/CEE357/CEE%20357%20air%20dispersion%20models.pdf

    let x = 0.5;         // 500 m downwind
    let y = 0.0;         // along plume centreline
    let z = 0.0;         // ground level
    let u_z = 6.0;      // 6 m/s wind speed at height of 50 m
    let pgcat = b'D';    // Neutral stability

    // Source centred on (0,0), height 50 m, 10 g/s mass emission rate
    let Q = 10.0;         //source.emission
    let H = 50.0;         //source.height

    // Calculate concentration at (x,y,z) == 19.2 ug/m3
    let s_y = get_sigma_y(pgcat, x);
    let s_z = get_sigma_z(pgcat, x);
    
    let conc = C(x, y, z, u_z, Q, H, s_y, s_z);
    assert!(approx_equal(conc, 1.917230120488e-05), "error");
}

#[test]
fn wind_components_test() {
    let source_x = -2.0;
    let source_y = -3.0;
    
    let sin_phi = 200.0_f64.to_radians().sin();
    let cos_phi = 200.0_f64.to_radians().cos();
    
    let (x, y) = wind_components(10.0, 10.0, source_x, source_y, sin_phi, cos_phi);

    assert!(approx_equal(x, 0.016320245790), "error");
    assert!(approx_equal(y, -6.8300495861972), "error");
}
