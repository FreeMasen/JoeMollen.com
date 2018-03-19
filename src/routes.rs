use std::fs::{read_dir, DirEntry};
use std::path::{PathBuf};
use futures::future::ok;
use hyper::server::{Request, Response};
use hyper::{StatusCode, Error};
use hyper::header::ContentLength;
use tera;
use futures::future::Future;

#[derive(Debug, Serialize)]
struct Project {
    pub name: String,
    pub images: Vec<String>,
}

pub fn index(_req: Request) -> Box<Future<Item = Response, Error = Error>> {
    let projects = get_projects();
    println!("{:?}", projects);
    let mut ctx = tera::Context::new();
    ctx.add("projects", &projects);
    let body = get_templates().render("index.html", &ctx).unwrap();
    Box::new(
        ok(
            Response::new()
                .with_status(StatusCode::Ok)
                .with_header(ContentLength(body.len() as u64))
                .with_body(body)
        )
    )
}   

fn get_templates() -> tera::Tera {
    compile_templates!("templates/**/*")
}

fn get_projects() -> Vec<Project> {
    let rd = if let Ok(r) = read_dir("portfolio") {
        r
    } else {
        return Vec::<Project>::new()
    };
    rd.map(from_path_to_project).collect()
}

fn from_path_to_project(de: Result<DirEntry, ::std::io::Error>) -> Project {
    let entry = if let Ok(e) = de {
        e
    } else {
        return Project {
            name: String::new(),
            images: vec!()
        }
    };
    let mut imgs_path = entry.path().clone();
    imgs_path.push("img");
    let name = from_path_to_string(Ok(entry));
    let mut images: Vec<String> = vec!();
    if let Ok(i_f) = read_dir(imgs_path) {
        for img in i_f {
            images.push(from_path_to_string(img))
        }
    };
    Project {
        name,
        images
    }
}

fn from_path_to_string(de: Result<DirEntry, ::std::io::Error>) -> String {
    let entry = if let Ok(e) = de {
        e
    } else {
        return String::new()
    };
    println!("entry: {:?}", entry);
    if let Ok(name) = entry.file_name().into_string() {
        println!("name: {:?}", name);
        name
    } else {
        String::new()
    }

}

pub fn page(req: Request) -> Box<Future<Item = Response, Error = Error>> {
    match req.uri().query() {
        Some(q) => {
            let parts: Vec<&str> = q.split("=").collect();
            if parts.len() > 2 || parts[0] != "name" {
                return bad_params()
            };
            let mut path = PathBuf::from("portfolio");
            path.push(parts[1]);
            let dir = if let Ok(mut dir) = read_dir(path) {
                if let Some(e) = dir.nth(0) {
                    e
                } else {
                    return bad_params()
                }
            } else {
                return bad_params()
            };
            let project = from_path_to_project(dir);
            let mut ctx = tera::Context::new();
            ctx.add("project", &project);
            let body = get_templates().render("index.html", &ctx).unwrap();
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