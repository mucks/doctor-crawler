use util;
use select::document::Document;
use select::predicate::{Name, Class, Attr};
use doctor::Doctor;

use std::thread;


pub fn get_all_doctors() -> Vec<Doctor> {
    let base = "https://www.gelbeseiten.de/frauenaerzte";
    
    let thread_count = 16;
    let mut count = 0;
    let mut threads = Vec::new();
    let mut all_doctors = Vec::new();

    let mut stop = false;
    
    for i in 0..1000 {
        if stop { break; }
        let url = format!("{}/s{}", base, i+1);
        threads.push(thread::spawn(move || {
            let doctors = get_doctors(&url);
            doctors
        }));
        if count >= thread_count {
            for t in threads {
                let doctors = t.join().unwrap();
                match doctors {
                    Ok(doctors) => all_doctors.extend(doctors),
                    Err(_) => stop = true
                }
                println!("{}", all_doctors.len());
            }
            threads = Vec::new();
            count = 0;
        }
        count += 1;
    }
    all_doctors
}

pub fn get_doctors(url: &str) -> Result<Vec<Doctor>, String> {
    let content = util::get_url_content_https_latin1(url);
    let document = Document::from(content.as_str());

    let mut doctors = Vec::new();
    let gs_treffer = match document.find(Attr("id", "gs_treffer")).next() {
        Some(gs) => gs,
        None => return Err("not found".into())
    };

    for article in gs_treffer.find(Class("teilnehmer")) {
        let name = match article.find(Class("teilnehmername")).next() {
            Some(a) => match a.find(Name("span")).next() {
                Some(span) => span.text(),
                None => "".into()
            },
            None => "".into()
        };
        
        let address = match article.find(Class("adresse")).next() {
            Some(address) => address,
            None => continue
        };

        let street_address = match address.find(Attr("itemprop","streetAddress")).next() {
            Some(span) => span.text(),
            None => "".into()
        };
        let zip_code = match address.find(Attr("itemprop", "postalCode")).next() {
            Some(span) => span.text(),
            None => "".into()
        };
        let city = match address.find(Attr("itemprop", "addressLocality")).next() {
            Some(span) => span.text(),
            None => "".into()
        };

        let phone = match article.find(Class("teilnehmertelefon")).next() {
            Some(div) => match div.find(Class("nummer")).next() {
                Some(span) => span.text(),
                None => "".into()
            }
            None => "".into()
        };

        let email = match article.find(Class("email_native_app")).next() {
            Some(a) => util::format_email(a.attr("href").unwrap_or("")),
            None => "".into()
        };

        let website = match article.find(Class("website")).next() {
            Some(div) => match div.find(Class("link")).next() {
                Some(a) => a.attr("href").unwrap_or(""),
                None => ""
            },
            None => ""
        };

        doctors.push(
            Doctor{
                name:name,
                address: street_address,
                zip_code: zip_code,
                city: city,
                phone: phone,
                email: email,
                website: website.into(),
                jameda_url: "not available".into()
            }
        );
    }
    Ok(doctors)
}
