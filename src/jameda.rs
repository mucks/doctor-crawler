use util;
use select::document::Document;
use select::predicate::{Name, Class, Attr};
use serde_json;
use serde_json::Value;
use doctor::{self, Doctor};

pub fn get_website(url: &str) -> String {
    let content = util::get_url_content_https(url);
    let document = Document::from(content.as_str());

    let profile = match document.find(Attr("id", "profil_name_adresse")).next() {
        Some(profile) => profile,
        None => return "not found".into()
    };

    for div in profile.find(Name("div")) {
        if div.find(Name("br")).count() == 1 {
            if div.find(Name("a")).count() == 1 {
                match div.find(Name("a")).next() {
                    Some(a) => {
                        let website = a.attr("href").unwrap();
                        if website.contains("http") { return website.into(); }
                        return "not found".into();
                    },
                    None => return "not found".into()
                }
            }
        }
    }
    return "not found".into()
}

fn get_websites(doctors: &Vec<Doctor>) -> Vec<Doctor> {
    let mut new_doctors = Vec::new();
    let mut threads = Vec::new();

    let doctor_chunks = doctor::split_array(&doctors);
    for chunk in doctor_chunks {
        let mut temp_chunk = chunk.clone();
        threads.push(thread::spawn(move || {
            for i in 0..chunk.len() {
                let website = get_website(&chunk[i].jameda_url);
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

pub fn get_doctors(url: &str) -> Vec<Doctor> {
    let content = util::get_url_content_https(url);
    let document = Document::from(content.as_str());


    let script = document
        .find(Name("body")).next().unwrap()
        .find(Name("script")).next().unwrap()
        .text();

    let mut json_string = script.split("window.__APP_STATE__=").collect::<Vec<&str>>()[1].to_owned();
    let len = json_string.len();
    json_string.truncate(len - 1);

    let json_value: Value = serde_json::from_str(&json_string).unwrap();

    let mut doctors = Vec::new();

    let results: Vec<Value> = match serde_json::from_str(&json_value["search"]["searchResult"]["results"].to_string()) {
        Ok(results) => results,
        Err(_) => return doctors
    };

    for result in results {
        let name = result["name_nice"].as_str().unwrap_or("not found").into();
        let address = result["strasse"].as_str().unwrap_or("not found").into();
        let city = result["ort"].as_str().unwrap_or("not found").into();
        let zip_code = result["plz"].as_str().unwrap_or("not found").into();
        let phone = result["tel"].as_str().unwrap_or("not found").into();
        let mut jameda_url = "jameda url not found".into();
        if let Some(url) = result["url"].as_str() {
            if let Some(url_hinten) = result["url_hinten"].as_str() {
                jameda_url = format!("https://www.jameda.de{}uebersicht/{}", url, url_hinten);
            }
        }
        doctors.push(Doctor { name: name, address: address, city: city, zip_code: zip_code, 
            phone: phone, 
            email: "".into(), 
            jameda_url: jameda_url,
            website: "".into()} );
    }
    doctors
}

pub fn get_cities() -> Vec<String> {
    let url = "https://www.jameda.de/arztsuche/fachgebiete/staedte/aerzte/frauenaerzte-gynaekologen";
    let content = util::get_url_content_https(url);
    let document = Document::from(content.as_str());

    let mut cities = Vec::new();

    let modul_box = document
        .find(Class("modul-box")).next().unwrap();

    for ul in modul_box.find(Name("ul")) { 
        for li in ul.find(Name("li")) {
            match li.find(Name("a")).next() {
                Some(a) => {
                    let city_url = a.attr("href").unwrap();
                    let _city_name = a.text();
                    cities.push(format!("https://www.jameda.de{}", city_url));
                }
                None => {}
            };
        }
    }
    cities
}


pub fn get_districts(url: &str) -> Vec<String> {
    let content = util::get_url_content_https(url);
    let document = Document::from(content.as_str());

    let mut districts = Vec::new();

    let div = match document.find(Class("sc-eMigcr")).next() {
        Some(div) => div,
        None => {
            districts.push(url.into());
            return districts
        }
    };

    for ul in div.find(Name("ul")) {
        for li in ul.find(Name("li")) {
            match li.find(Class("sc-cpmLhU")).next() {
                Some(a) => {
                    let district_url = a.attr("href").unwrap();
                    districts.push(format!("https://www.jameda.de{}", district_url));
                },
                None => {
                    //println!("district not found");
                }
            }
        }
    }
    districts
}


use thread;

pub fn get_all_doctors() -> Vec<Doctor> {
    let thread_count = 16;

    let cities = get_cities();
    let mut threads = Vec::new();
    for city in cities {
        threads.push(thread::spawn(move || {
            let districts = get_districts(&city);
            districts
        }));
    }

    let mut all_districts = Vec::new();
    let mut all_doctors = Vec::new();

    for t in threads {
        let districts = t.join().unwrap();
        all_districts.extend(districts);
    }

    let mut more_threads = Vec::new();

    let mut count = 0;

    for district in all_districts {
        more_threads.push(thread::spawn(move || {
            let doctors = get_doctors(&district);
            doctors
        }));

        if count >= thread_count {
            for t in more_threads {
                let doctors = t.join().unwrap();
                all_doctors.extend(doctors);
                println!("{}", all_doctors.len());
            }
            more_threads = Vec::new();
            count = 0;
        }
        count += 1;
    };

    all_doctors
}

