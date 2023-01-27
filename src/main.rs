#[macro_use] extern crate rocket;

mod pasha_id;

use pasha_id::PashaId;
use rocket::tokio::fs::File;
use std::path::Path;

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
async fn retreive(id: &str) -> Option<File> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
    let filename = Path::new(upload_dir).join(id);
    File::open(&filename).await.ok()
}
    
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retreive])
}

