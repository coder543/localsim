extern crate rustc_serialize;
extern crate csv;
extern crate stopwatch;
extern crate bincode;
extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;

pub mod sim_data;
pub use sim_data::Star;

use stopwatch::Stopwatch;

use std::io::{stdin, stdout, BufRead, Write};

fn load_database() -> Result<Vec<Star>, &'static str> {

    let reader_wrap = csv::Reader::from_file("hygdata_v3.csv");
    let mut reader;
    if let Ok(reader_val) = reader_wrap {
        reader = reader_val;
    } else {
        return Err("Unable to read csv!");
    }
    let sw = Stopwatch::start_new();
    println!("Initializing database from csv...");

    let stars_wrap: csv::Result<Vec<Star>> = reader.decode().collect::<csv::Result<Vec<Star>>>();

    if let Ok(stars) = stars_wrap {
        println!("Loading took {} seconds\n",
                 sw.elapsed_ms() as f32 / 1000f32);
        return Ok(stars);
    } else {
        return Err("Unable to decode csv file!");
    }
}


//translated from http://stackoverflow.com/a/22630970/389837
fn bv2rgb(mut bv: f64) -> (f32, f32, f32) {
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
    return (r as f32, g as f32, b as f32);
}

fn main() {
    let maxdist: f64;
    let stars = load_database().unwrap();
    let stdin = stdin();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let line = line.trim();
        if line == "q" {
            return;
        } else {
            let maxdist_wrap = line.parse::<f64>();
            if let Ok(dist) = maxdist_wrap {
                maxdist = dist * 0.306601;
                break;
            } else {
                println!("Please enter either 'q' to quit, or a distance in lightyears.");
            }
        }
    }
    let sw = Stopwatch::start_new();
    let mut named_vec = Vec::new();
    for star in stars {
        // let Some(mscatid) = star.multistar_catalog_id.clone()
        if star.distance < maxdist {
            named_vec.push(star.clone());
        }
    }
    named_vec.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    let starcount = named_vec.len();
    let mut window = Window::new(&format!("Stars within {:.3} lightyears", maxdist * 3.26156));
    window.set_framerate_limit(Some(75));
    window.set_light(Light::StickToCamera);
    for star in named_vec {
        let mut sph = window.add_sphere(0.015);
        if star.id == 0 {
            sph.set_color(1.0, 1.0, 0.7);
        } else {
            if let Some(bv) = star.color_index {
                let (r, g, b) = bv2rgb(bv);
                sph.set_color(r, g, b);
            } else {
                //use a green color to denote that the color isn't known, since green stars don't exist
                //sph.set_color(0.0, 1.0, 0.0);

                //use a white color to look more natural, but still slightly unnatural.
                sph.set_color(1.0, 1.0, 1.0);
            }
        }
        sph.append_translation(&Vector3::new(star.x as f32, star.y as f32, star.z as f32));
        println!("{:.3} lightyears\n{:?}\n\n", star.distance * 3.26156, star);
    }
    println!("There were {} matching stars.", starcount);
    println!("Processing took {} seconds",
             sw.elapsed_ms() as f32 / 1000f32);
    while window.render() {}
    window.hide();
}
