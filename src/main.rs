extern crate hyper;
extern crate futures;
#[macro_use]
extern crate tera;
extern crate pony;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate percent_encoding;
extern crate pulldown_cmark;

use std::env::args;
use std::path::PathBuf;
use hyper::server::{NewService, Http};

mod routes;
mod page;
mod writer;

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() > 1 && &args[1] == "static" {
        let path = args.pop().expect("Unable to get path");
        write(path);
    } else {
        serve();
    }
}

fn write(path: String) {
    println!("writing to {:?}", &path);
    writer::write(PathBuf::from(&path));
}

fn serve() {
    let mut pb = pony::pony_builder::PonyBuilder::new();
    pb.get("/", routes::index);
    pb.get("/page", routes::page);
    pb.get("/about", routes::about);
    pb.get("/contact", routes::contact);
    pb.post("/message", routes::message);
    pb.use_static("www/");
    pb.use_static_logging();
    let addr = "127.0.0.1:9990".parse().unwrap();
    let handler = Http::new().bind(&addr, move || pb.new_service()).unwrap();
    let _ = handler.run();
}
