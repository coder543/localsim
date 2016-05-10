#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct Star
{
    //various means of ID
    pub id: u32,
    pub hipparcos: Option<u32>,
    pub henry_draper: Option<u32>,
    pub harvard_revised: Option<u32>,
    pub gliese: Option<u32>,
    pub bayer_flamsteed: Option<String>,
    pub common_name: Option<String>,
    
    //physical properties
    pub ra: f64,
    pub dec: f64,
    pub distance: f64, //unit is parsecs
    pub pmra: f64, //proper motion, right ascension  milliarcseconds per year
    pub pmdec: f64,//proper motion, declination,     milliarcseconds per year
    pub radial_velocity: Option<f64>,
    pub magnitude: f64, //apparent visual magnitude
    pub abs_magnitude: f64, //apparent mag normalized to 10 parsecs
    pub spectral_type: Option<String>,
    pub color_index: Option<f64>,
    pub x: f64, //unit is parsecs
    pub y: f64, //unit is parsecs
    pub z: f64, //unit is parsecs
    pub vx: f64, //velocity on x axis, unit is parsecs per year
    pub vy: f64, //velocity on y axis, unit is parsecs per year
    pub vz: f64, //velocity on z axis, unit is parsecs per year
    pub rarad: f64,  //ra in radians
    pub decrad: f64, //dec in radians
    pub pmrarad: f64, //proper motion in radians per year
    pub pmdecrad: f64, //proper motion in radians per year
    pub bayer: Option<String>,
    pub flam: Option<u32>,
    pub constellation: Option<String>,
    
    //Gliese multi-star system information
    pub companion_id: i32,
    pub primary_star_id: i32,
    pub multistar_catalog_id: Option<String>,
    
    //misc physical properties
    pub lum: f64, //luminosity as a multiple of solar luminosity
    pub var: Option<String>,  //variable star designation
    pub var_min: Option<f64>, //approximate minimum magnitude for variable star
    pub var_max: Option<f64>  //approximate maximum magnitude for variable star
}