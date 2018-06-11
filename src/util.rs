use hyper::{Uri, Client};
use futures::future::Future;
use futures::Stream;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::Request;
use hyper::header::UserAgent;
use hyper::Method::Get;


pub fn format_email(email_with_mailto: &str) -> String {
    let preceding = email_with_mailto.split("mailto:").collect::<Vec<&str>>();

    if preceding.len() >= 2 {
        let following = preceding[1].split("?subject").collect::<Vec<&str>>();
        if following.len() >= 1 {
            return following[0].into()
        }
    }
    email_with_mailto.into()
}

pub fn get_url_content(url: &str) -> String {
    let uri = url.parse::<Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let work = client.get(uri).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
    });
    core.run(work).unwrap()
}

pub fn get_url_content_https(url: &str) -> String {
    let uri = url.parse::<Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let mut req = Request::new(Get, uri);
    req.headers_mut().set(UserAgent::new("Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0"));

    let work = client.request(req).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
    });
    core.run(work).unwrap()
}

pub fn get_url_content_https_latin1(url: &str) -> String {
    let uri = url.parse::<Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let mut req = Request::new(Get, uri);
    req.headers_mut().set(UserAgent::new("Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0"));

    let work = client.request(req).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            latin1_to_string(&v)
        })
    });
    core.run(work).unwrap()
}
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}
