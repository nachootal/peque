//! Serve webpages from the URL!
//!
//! This crate serves either a landing page when called to the base url or takes the rest of the
//! url as a base64 gzipped web page and serves it.
//!
//Use following de/encoder: https://base64.guru/standards/base64url/encode
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::get;
use base64_url;
use std::io::Read;
use libflate::gzip::Decoder;
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;
#[get("/<base64_gzip_website>")]
fn websites(base64_gzip_website: &str) -> status::Custom<content::RawHtml<String>> {
    // Decode from BASE64
    match base64_url::decode(base64_gzip_website) {
        Ok(gzip_website) => {
            // Check the content of the website - It can be GZIP or assume Plain
            match Decoder::new(&gzip_website[..]) {
                Ok(mut decoder) => {
                    let mut website = String::new();
                    decoder.read_to_string(&mut website).unwrap();
                    return status::Custom(Status::Ok, content::RawHtml(website))
                },
                Err(_) => return status::Custom(Status::Ok, content::RawHtml(String::from_utf8(gzip_website).unwrap())),
            }
        },
        Err(e) => {
            return status::Custom(Status::UnprocessableEntity , content::RawHtml(e.to_string()))
        },
    };
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {text: "Hello and welcome to the peque site :)"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![websites, index])
        .attach(Template::fairing())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn test_websites_responds_with_error_when_wrong_input() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::websites("Invalid"))).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert_eq!("Invalid last symbol 100, offset 6.", response.into_string().unwrap())
    }
    #[test]
    fn test_websites_responds_with_site_when_correct_base_coding_input() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::websites("VmFsaWQK"))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!("Valid\n", response.into_string().unwrap())
    }
    #[test]
    fn test_websites_responds_with_site_when_correct_base_coding_and_gzip_input() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::websites("H4sICGQxHWUAA2hlbGxvLnBsYWluAPNIzcnJVwjPL8pJ4QIA4-WVsAwAAAA"))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!("Hello World\n", response.into_string().unwrap())
    }
}
