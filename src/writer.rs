use std::io::{Write, Read};
use std::fs::{File, DirBuilder};
use std::path::PathBuf;
use page::{get_all_pages, Page};
use tera::{Tera, Value, Result, Context};
use std::collections::HashMap;

use pulldown_cmark::html::push_html;
use pulldown_cmark::Parser;

pub fn write(path: PathBuf) {
    let mut path = path.clone();
    let pages = get_all_pages().expect("Unable to get pages");
    let db = DirBuilder::new();
    db.create(&path).expect(&format!("Unable to create base folder: {:?}", &path));
    if let Some(html) = index_html(&pages) {
        path.push("index.html");
        let mut index = File::create(&path).expect("unable to create index.html");
        index.write_all(html.as_bytes()).expect("Unable to write html to index.html");
    }
    //remove index.html
    let _ = path.pop();
    for page in pages {
        let mut page_path = path.join(&page.project.name);
        if let Err(_e) = db.create(&page_path) {
            println!("unable to create: {:?}", &page_path);
            continue;
        };
        if let Some(html) = page_html(&page) {
            page_path.push("index.html");
            let mut index = File::create(&page_path).expect(&format!("Unable to create {:?}", &page_path));
            index.write_all(html.as_bytes()).expect(&format!("Unable to write html for {:?}", &page_path));
        }
        //todo: move all images and create the img folder?
    }
}

pub fn page_html(page: &Page) -> Option<String> {
        let mut ctx = Context::new();
        ctx.add("page", page);
        match get_templates().render("page.html", &ctx) {
            Ok(html) => Some(html),
            Err(e) => {
                println!("Error getting template html: {:?}", e);
                None
            }
        }
}

pub fn index_html(pages: &Vec<Page>) -> Option<String> {
    let mut ctx = Context::new();
    ctx.add("pages", pages);
    match get_templates().render("index.html", &ctx) {
        Ok(body) => Some(body),
        Err(e) => {
            println!("Error rending html: {:?}", e);
            None
        }
    }
}

pub fn about_html() -> Option<String> {
    let mut ctx = Context::new();
    let mut content = String::new();
    match File::open("www/about.md") {
        Ok(mut f) => {
            match f.read_to_string(&mut content) {
                Ok(_size) => {
                    let html = md_to_html(&content);
                    ctx.add("content", &html);
                }, 
                Err(e) => println!("Error reading content {:?}", e),
            }
        },
        Err(e) => println!("Error opening about content {:?}", e),
    };
    match get_templates().render("about.html", &ctx) {
        Ok(body) => Some(body),
        Err(e) => {
            println!("Error rendering html: {:?}", e);
            None
        }
    }
}

pub fn contact_html() -> Option<String> {
    let ctx = Context::new();
    match get_templates().render("contact.html", &ctx) {
        Ok(body) => Some(body),
        Err(e) => {
            println!("Error rendering html: {:?}", e);
            None
        }
    }
}

fn get_templates() -> Tera {
    let mut t = compile_templates!("templates/**/*");
    t.register_filter("convert_numeric", convert_numeric);
    t
}

fn convert_numeric(value: Value, _: HashMap<String, Value>) -> Result<Value> {
    println!("convert_numeric {:?}", value);
    match value {
        Value::String(mut text) => {
            println!("text: {:?}", text);
            let numbers = numbers();
            for (n, t) in numbers {
                text = text.replace(n, &t);
            }
            Ok(Value::String(text))
        },
        _ => Ok(value)
    }
}

fn numbers() -> HashMap<char, String> {
    let mut ret = HashMap::new();
    ret.insert('0', String::from("zero"));
    ret.insert('1', String::from("one"));
    ret.insert('2', String::from("two"));
    ret.insert('3', String::from("three"));
    ret.insert('4', String::from("four"));
    ret.insert('5', String::from("five"));
    ret.insert('6', String::from("six"));
    ret.insert('7', String::from("seven"));
    ret.insert('8', String::from("eight"));
    ret.insert('9', String::from("nine"));
    ret
}

pub fn md_to_html(html: &str) -> String {
    let parser = Parser::new(html);
    let mut buf = String::new();
    push_html(&mut buf, parser);
    buf
}