// use futures::future::ok;
// use hyper::server::{Request, Response};
// use hyper::{Error, StatusCode};
// use hyper::header::{ContentLength, Location};
// use futures::future::Future;
// use page::{get_all_pages, get_page};
// use writer::{page_html, index_html, about_html, contact_html};
// use percent_encoding::percent_decode;

// pub fn index(_req: Request) -> Box<Future<Item = Response, Error = Error>> {
//     match get_all_pages() {
//         Some(p) => match index_html(&p) {
//             Some(body) => Box::new(ok(Response::new()
//                 .with_status(StatusCode::Ok)
//                 .with_header(ContentLength(body.len() as u64))
//                 .with_body(body))),
//             None => not_found(),
//         },
//         None => not_found(),
//     }
// }

// pub fn page(req: Request) -> Box<Future<Item = Response, Error = Error>> {
//     match req.uri().query() {
//         Some(q) => {
//             let parsed = match percent_decode(q.as_bytes()).decode_utf8() {
//                 Ok(p) => p.into_owned(),
//                 Err(_e) => String::new(),
//             };
//             let parts: Vec<&str> = parsed.split("=").collect();
//             if parts.len() > 2 || parts[0] != "name" {
//                 return bad_params();
//             };
//             let page = if let Some(page) = get_page(parts[1]) {
//                 page
//             } else {
//                 return bad_params();
//             };
//             let body = if let Some(b) = page_html(&page) {
//                 b
//             } else {
//                 return bad_params();
//             };
//             Box::new(ok(Response::new()
//                 .with_status(StatusCode::Ok)
//                 .with_header(ContentLength(body.len() as u64))
//                 .with_body(body)))
//         }
//         _ => bad_params(),
//     }
// }

// pub fn about(_req: Request) -> ::pony::HyperResult {
//     let body = if let Some(b) = about_html()  {
//         b
//     } else {
//         return not_found();
//     };
//     Box::new(ok(Response::new()
//                 .with_status(StatusCode::Ok)
//                 .with_header(ContentLength(body.len() as u64))
//                 .with_body(body)))
// }

// pub fn contact(_req: Request) -> ::pony::HyperResult {
//     let body = if let Some(b) = contact_html() {
//         b
//     } else {
//         return not_found()
//     };
//     Box::new(ok(Response::new()
//                 .with_status(StatusCode::Ok)
//                 .with_header(ContentLength(body.len() as u64))
//                 .with_body(body)))
// }

// pub fn message(_req: Request) -> ::pony::HyperResult {
//     //TODO: capture form elements?
//     Box::new(
//         ok(
//             Response::new()
//                 .with_status(StatusCode::PermanentRedirect)
//                 .with_header(Location::new("/contact"))
//         )
//     )
// }

// fn bad_params() -> Box<Future<Item = Response, Error = Error>> {
//     Box::new(ok(Response::new()
//         .with_status(StatusCode::UnprocessableEntity)))
// }

// fn not_found() -> Box<Future<Item = Response, Error = Error>>{
//     Box::new(ok(Response::new()
//         .with_status(StatusCode::NotFound)))
// }