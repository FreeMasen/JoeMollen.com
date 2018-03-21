use futures::future::ok;
use hyper::server::{Request, Response};
use hyper::{StatusCode, Error};
use hyper::header::ContentLength;
use tera;
use futures::future::Future;
use page::{get_all_pages, get_page};
use percent_encoding::percent_decode;

pub fn index(_req: Request) -> Box<Future<Item = Response, Error = Error>> {
    let pages = if let Some(p) = get_all_pages() {
        p
    } else {
        return bad_params()
    };
    let mut ctx = tera::Context::new();
    ctx.add("pages", &pages);
    match get_templates().render("index.html", &ctx) {
        Ok(body) => {
            Box::new(
                ok(
                    Response::new()
                        .with_status(StatusCode::Ok)
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body)
                )
            )
        },
        Err(e) => {
            println!("template error {:?}", e);
            bad_params()
        }
    }

}   

fn get_templates() -> tera::Tera {
    compile_templates!("templates/**/*")
}

pub fn page(req: Request) -> Box<Future<Item = Response, Error = Error>> {
    match req.uri().query() {
        Some(q) => {
            println!("q: {:?}", q);
            let parsed = match percent_decode(q.as_bytes()).decode_utf8() {
                Ok(p) => p.into_owned(),
                Err(_e) => String::new()
            };
            let parts: Vec<&str> = parsed.split("=").collect();
            println!("parts: {:?}", parts);
            if parts.len() > 2 || parts[0] != "name" {
                return bad_params()
            };
            let page = if let Some(p) = get_page(parts[1]) {
                p
            } else {
                return bad_params()
            };
            println!("page: {:?}", page);
            let mut ctx = tera::Context::new();
            ctx.add("page", &page);
            let body = if let Ok(b) = get_templates().render("page.html", &ctx) {
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

fn bad_params() -> Box<Future<Item = Response, Error = Error>> {
    Box::new(
        ok(
            Response::new()
                .with_status(StatusCode::UnprocessableEntity)
        )
    )
}