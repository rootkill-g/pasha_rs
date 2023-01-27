#[macro_use]
extern crate rocket;

mod pasha_id;

use pasha_id::PashaId;
use rocket::{data::{Data, ToByteUnit}, http::uri::Absolute, tokio::fs::File};
use std::io;

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[get("/")]
async fn index() -> &'static str {
    "This is the index page for PASHA.rs
    USAGE:
        POST /
            accepts raw data in the body of the request and responds 
            with a URL of a page containing the body's content

        GET /<id>
            retreives the content for the paste with id `<id>`
"
}

#[get("/<id>")]
async fn retrieve(id: PashaId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[post("/", data = "<pasha>")]
async fn uploads(pasha: Data<'_>) -> std::io::Result<String> {
    let id = PashaId::new(ID_LENGTH);
    pasha.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve, uploads])
}
