extern crate hyper;
extern crate futures;
#[macro_use]
extern crate tera;
extern crate pony;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use hyper::server::{NewService, Http};

mod routes;
mod page;

fn main() {
    let mut pb = pony::pony_builder::PonyBuilder::new();
    pb.get("/", routes::index);
    pb.get("/page", routes::page);
    pb.use_static("portfolio/");
    let addr = "127.0.0.1:1111".parse().unwrap();
    let handler = Http::new().bind(&addr, move || pb.new_service()).unwrap();
    let _ = handler.run();
}
