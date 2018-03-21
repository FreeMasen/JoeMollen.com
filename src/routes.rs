use futures::future::ok;
use hyper::server::{Request, Response};
use hyper::{StatusCode, Error};
use hyper::header::ContentLength;
use tera;
use futures::future::Future;
use page::{get_all_pages, get_page};
use percent_encoding::percent_decode;

pub fn index(_req: Request) -> Box<Future<Item = Response, Error = Error>> {
    match index_html() {
        Some(body) => {
            Box::new(
                ok(
                    Response::new()
                        .with_status(StatusCode::Ok)
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body)
                )
            )
        },
        None => {
            Box::new(
                ok(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                )
            )
        }
    }
}

fn index_html() -> Option<String> {
    match get_all_pages() {
        Some(p) => {
            let mut ctx = tera::Context::new();
            ctx.add("pages", &p);
            match get_templates().render("index.html", &ctx) {
                Ok(body) => {
                    Some(body)
                },
                Err(e) => {
                    println!("Error rending html: {:?}", e);
                    None
                }
            }
        },
        _ => None
    }
}

fn get_templates() -> tera::Tera {
    compile_templates!("templates/**/*")
}

pub fn page(req: Request) -> Box<Future<Item = Response, Error = Error>> {
    match req.uri().query() {
        Some(q) => {
            let parsed = match percent_decode(q.as_bytes()).decode_utf8() {
                Ok(p) => p.into_owned(),
                Err(_e) => String::new()
            };
            let parts: Vec<&str> = parsed.split("=").collect();
            if parts.len() > 2 || parts[0] != "name" {
                return bad_params()
            };

            let body = if let Some(b) = page_html(parts[1]) {
                b
            } else {
                return bad_params()
            };
            Box::new(
                ok(
                    Response::new()
                        .with_status(StatusCode::Ok)
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body)
                )
            )
        },
        _ => bad_params()
    }
}

fn page_html(name: &str) -> Option<String> {
    match get_page(name) {
        Some(p) => {
            let mut ctx = tera::Context::new();
            ctx.add("page", &p);
            match get_templates().render("page.html", &ctx) {
                Ok(html) => Some(html),
                Err(e) => {
                    println!("Error getting template html: {:?}", e);
                    None
                }
            }
        },
        _ => None
    }
}

fn bad_params() -> Box<Future<Item = Response, Error = Error>> {
    Box::new(
        ok(
            Response::new()
                .with_status(StatusCode::UnprocessableEntity)
        )
    )
}