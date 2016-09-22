#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct Star {
    // various means of ID
    pub id: u32,
    pub hipparcos: Option<u32>,
    pub henry_draper: Option<u32>,
    pub harvard_revised: Option<u32>,
    pub gliese: Option<u32>,
    pub bayer_flamsteed: Option<String>,
    pub common_name: Option<String>,

    // physical properties
    pub ra: f64,
    pub dec: f64,
    pub distance: f64, // unit is parsecs
    pub pmra: f64, // proper motion, right ascension  milliarcseconds per year
    pub pmdec: f64, // proper motion, declination,     milliarcseconds per year
    pub radial_velocity: Option<f64>,
    pub magnitude: f64, // apparent visual magnitude
    pub abs_magnitude: f64, // apparent mag normalized to 10 parsecs
    pub spectral_type: Option<String>,
    pub color_index: Option<f64>,
    pub x: f64, // unit is parsecs
    pub y: f64, // unit is parsecs
    pub z: f64, // unit is parsecs
    pub vx: f64, // velocity on x axis, unit is parsecs per year
    pub vy: f64, // velocity on y axis, unit is parsecs per year
    pub vz: f64, // velocity on z axis, unit is parsecs per year
    pub rarad: f64, // ra in radians
    pub decrad: f64, // dec in radians
    pub pmrarad: f64, // proper motion in radians per year
    pub pmdecrad: f64, // proper motion in radians per year
    pub bayer: Option<String>,
    pub flam: Option<u32>,
    pub constellation: Option<String>,

    // Gliese multi-star system information
    pub companion_id: i32,
    pub primary_star_id: i32,
    pub multistar_catalog_id: Option<String>,

    // misc physical properties
    pub lum: f64, // luminosity as a multiple of solar luminosity
    pub var: Option<String>, // variable star designation
    pub var_min: Option<f64>, // approximate minimum magnitude for variable star
    pub var_max: Option<f64>, // approximate maximum magnitude for variable star
}


impl Star {
    // translated from http://stackoverflow.com/a/22630970/389837
    pub fn bv2rgb_opt(&self) -> Option<(f32, f32, f32)> {
        self.color_index.map(|_| self.bv2rgb())
    }

    pub fn bv2rgb(&self) -> (f32, f32, f32) {
        let mut bv = self.color_index.unwrap();
        let r: f64;
        let g: f64;
        let b: f64;
        let mut t: f64;
        if bv < -0.4 {
            bv = -0.4;
        }
        if bv > 2.0 {
            bv = 2.0;
        }
        if (bv >= -0.40) && (bv < 0.00) {
            t = (bv + 0.40) / (0.00 + 0.40);
            r = 0.61 + (0.11 * t) + (0.1 * t * t);
        } else if (bv >= 0.00) && (bv < 0.40) {
            t = (bv - 0.00) / (0.40 - 0.00);
            r = 0.83 + (0.17 * t);
        } else if (bv >= 0.40) && (bv < 2.10) {
            r = 1.00;
        } else {
            r = 1.0;
        }
        if (bv >= -0.40) && (bv < 0.00) {
            t = (bv + 0.40) / (0.00 + 0.40);
            g = 0.70 + (0.07 * t) + (0.1 * t * t);
        } else if (bv >= 0.00) && (bv < 0.40) {
            t = (bv - 0.00) / (0.40 - 0.00);
            g = 0.87 + (0.11 * t);
        } else if (bv >= 0.40) && (bv < 1.60) {
            t = (bv - 0.40) / (1.60 - 0.40);
            g = 0.98 - (0.16 * t);
        } else if (bv >= 1.60) && (bv < 2.00) {
            t = (bv - 1.60) / (2.00 - 1.60);
            g = 0.82 - (0.5 * t * t);
        } else {
            g = 1.0;
        }
        if (bv >= -0.40) && (bv < 0.40) {
            b = 1.00;
        } else if (bv >= 0.40) && (bv < 1.50) {
            t = (bv - 0.40) / (1.50 - 0.40);
            b = 1.00 - (0.47 * t) + (0.1 * t * t);
        } else if (bv >= 1.50) && (bv < 1.94) {
            t = (bv - 1.50) / (1.94 - 1.50);
            b = 0.63 - (0.6 * t * t);
        } else {
            b = 1.0;
        }

        (r as f32, g as f32, b as f32)
    }
}