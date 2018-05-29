extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate select;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate csv;

mod util;
mod doctor;
mod jameda;

use std::process::Command;
use std::thread;
use std::fs::File;

fn main() {
    //let doctors = jameda::get_doctors("https://www.jameda.de/search/berlin/tiergarten/aerzte/frauenaerzte-gynaekologen/praxis/");
    let doctors = jameda::get_all_doctors();

    let file = File::create("doctors.csv").unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for doctor in doctors {
        wtr.serialize(doctor).unwrap();
    }
    wtr.flush().unwrap();
}

fn puppeteer() {
    let output = if cfg!(target_os = "windows") { 
        Command::new("assets/nodejs/node-v10.2.1-win-x64/node.exe")
            .arg("assets/doctor_finder_node/src/main.js")
            .output()
            .expect("failed to run nodejs")
    } else {
        Command::new("assets/nodejs/node-v10.2.1-linux-x64/bin/node")
            .arg("assets/doctor_finder_node/src/main.js")
            .output()
            .expect("failed to run nodejs")
    };

    let content = String::from_utf8(output.stdout).unwrap();
    println!("{}", content);
}
