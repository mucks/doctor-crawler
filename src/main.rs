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
use doctor::Doctor;

fn main() {
    let doctors = read_file("out/sorted_doctors.csv");
    let doctors_with_websites = get_websites(&doctors);
    write_file("out/doctors_with_websites.csv", &doctors_with_websites);
}

fn get_websites(doctors: &Vec<Doctor>) -> Vec<Doctor> {
    let mut new_doctors = Vec::new();
    let mut threads = Vec::new();

    let doctor_chunks = doctor::split_array(&doctors);
    for chunk in doctor_chunks {
        let mut temp_chunk = chunk.clone();
        threads.push(thread::spawn(move || {
            for i in 0..chunk.len() {
                let website = jameda::get_website(&chunk[i].jameda_url);
                println!("{}: {}", i, website);
                temp_chunk[i].website = website;
            }
            temp_chunk
        }));
    }

    for t in threads {
        new_doctors.extend(t.join().unwrap());
    }
    new_doctors
}

fn write_file(path: &str, doctors: &Vec<doctor::Doctor>) {
    let file = File::create(path).unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for doctor in doctors {
        wtr.serialize(doctor).unwrap();
    }
    wtr.flush().unwrap();
}


fn read_file(path: &str) -> Vec<doctor::Doctor> {
    let mut doctors = Vec::new();
    
    let file = File::open(path).unwrap();

    let mut rtr = csv::Reader::from_reader(file);

    for result in rtr.deserialize() {
        let doctor = result.unwrap(); 
        doctors.push(doctor);
    }
    doctors
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
