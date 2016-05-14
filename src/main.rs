extern crate rustc_serialize;
extern crate csv;
extern crate stopwatch;
extern crate bincode;
extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;

mod sim_data;
use sim_data::Star;

use stopwatch::Stopwatch;

use std::io::{stdin,stdout,BufRead,Write};

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
        println!("Loading took {} seconds\n", sw.elapsed_ms() as f32 / 1000f32);
        return Ok(stars);
    } else {
        return Err("Unable to decode csv file!");
    }
}

fn main() {
    let maxdist: f64;
    let stars = load_database().unwrap();
    let stdin = stdin();
    // loop {
        let stars = stars.clone();
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
            //let Some(mscatid) = star.multistar_catalog_id.clone()
            if star.distance < maxdist {
                named_vec.push(star.clone());
            }
        }
        named_vec.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        let starcount = named_vec.len();
        let mut window = Window::new(&format!("Stars within {:.3} lightyears", maxdist * 3.26156));
        window.set_light(Light::StickToCamera);
        for star in named_vec {
            let mut sph = window.add_sphere(0.01);
            if star.id == 0 {
                sph.set_color(1.0, 1.0, 0.7);
            } else {
                sph.set_color(0.7, 0.7, 1.0);
            }
            sph.append_translation(&Vector3::new(star.x as f32, star.y as f32, star.z as f32));
            println!("{:.3} lightyears\n{:?}\n\n", star.distance * 3.26156, star);
        }
        println!("There were {} matching stars.", starcount);
        println!("Processing took {} seconds", sw.elapsed_ms() as f32 / 1000f32);
        while window.render() {}
        window.hide();
    // }
}   