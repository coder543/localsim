extern crate rustc_serialize;
extern crate csv;
extern crate stopwatch;
extern crate bincode;

mod sim_data;
use sim_data::Star;

use stopwatch::Stopwatch;

use std::fs::File;

fn save_database(stars: &[Star]) {
    println!("Saving database 'stars.bin' to disk...");
    let sw = Stopwatch::start_new();
    let binfile = File::create("stars.bin");
    if let Ok(mut binfile) = binfile {
        bincode::rustc_serialize::encode_into(&stars, &mut binfile, bincode::SizeLimit::Infinite).unwrap();
        println!("Saved. took {} seconds", sw.elapsed_ms() as f32 / 1000f32);
    } else {
        println!("Could not create stars.bin!");
    }
}

fn load_database() -> Result<Vec<Star>, &'static str> {
    let binfile = File::open("stars.bin");
    if let Ok(mut binfile) = binfile {
        println!("Loading database from bin...");
        let sw = Stopwatch::start_new();
        let stars_wrap = bincode::rustc_serialize::decode_from(&mut binfile, bincode::SizeLimit::Infinite);
        if let Ok(stars) = stars_wrap {
            println!("Loading took {} seconds", sw.elapsed_ms() as f32 / 1000f32);
            return Ok(stars);
        } else {
            return Err("Unable to parse 'stars.bin'");
        }
    } else {
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
            println!("Loading took {} seconds", sw.elapsed_ms() as f32 / 1000f32);
            save_database(&stars);
            return Ok(stars);
        } else {
            return Err("Unable to decode csv file!");
        }
    }
    // return Ok(stars);
}

fn main() {
    let stars = load_database().unwrap();
    let sw = Stopwatch::start_new();
    let mut named_vec = Vec::new();
    for star in stars {
        if let Some(common_name) = star.common_name.clone() {
            named_vec.push(star.clone());
            println!("{}, {:?}", common_name, star);
        }
    }
    println!("There were {} named stars.", named_vec.len());
    println!("Processing took {} seconds", sw.elapsed_ms() as f32 / 1000f32);
}   