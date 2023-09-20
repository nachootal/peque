use rocket::http::Status;
use rocket::response::{content, status};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use libflate::gzip::Decoder;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}
#[get("/<website>")]
fn websites(website: &str) -> status::Custom<content::RawHtml<String>> {
    let base64_gzip_website;
    // Decode from BASE64
    match general_purpose::STANDARD.decode(website) {
        Ok(decoded) => {
            base64_gzip_website = decoded.clone();
            // Decode the string
            let gzip_website = std::str::from_utf8(&base64_gzip_website).unwrap_or("Hello World!");
            // Inflate the GZIP
            // let mut decoder = Decoder::new(&gzip_website).unwrap();
            // let mut website = Vec::new();
            // decoder.read_to_end(&mut website).unwrap();
            // return status::Custom(Status::Accepted , content::RawHtml(format!("{}", website)))
            return status::Custom(Status::Accepted , content::RawHtml(format!("{}", gzip_website)))
        },
        Err(e) => {
            return status::Custom(Status::Accepted , content::RawHtml(e.to_string()))
        },
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![websites, index])
}
