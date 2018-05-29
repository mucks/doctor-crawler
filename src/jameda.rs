use util;
use select::document::Document;
use select::predicate::{Name, Class, Attr};
use serde_json;
use serde_json::Value;
use doctor::Doctor;



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

    let results: Vec<Value> = serde_json::from_str(&json_value["search"]["searchResult"]["results"].to_string()).unwrap();

    let mut doctors = Vec::new();

    for result in results {
        let name = result["name_nice"].to_string();
        let address = result["strasse"].to_string();
        let city = result["ort"].to_string();
        let zip_code = result["plz"].to_string();
        let phone = result["tel"].to_string();
        doctors.push(Doctor { name: name, address: address, city: city, zip_code: zip_code, phone: phone, email: "".into(), website: "".into() } );
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

    let div = match document.find(Class("sc-bHwgHz")).next() {
        Some(div) => div,
        None => {
            districts.push(url.into());
            return districts
        }
    };

    for ul in div.find(Name("ul")) {
        for li in ul.find(Name("li")) {
            let a = li.find(Class("sc-gHboQg")).next().unwrap();
            let district_url = a.attr("href").unwrap();
            districts.push(format!("https://www.jameda.de{}", district_url));
        }
    }
    districts
}


use thread;

pub fn get_all_doctors() -> Vec<Doctor> {
    let THREAD_COUNT = 16;

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

        if count >= THREAD_COUNT {
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

