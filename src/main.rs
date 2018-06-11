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
mod gs;

use std::process::Command;
use std::thread;
use std::fs::File;
use doctor::Doctor;

fn main() {
    let doctors = doctor::remove_duplicates(&gs::get_all_doctors());
    write_file("out/gs_doctors.csv", &doctors);
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
