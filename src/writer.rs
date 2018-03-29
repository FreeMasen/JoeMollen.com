use std::io::{Write, Read};
use std::fs::{File, DirBuilder, copy, read_dir};
use std::ops::Add;
use std::path::PathBuf;
use page::{get_all_pages, Page};
use tera::{Tera, Value, Result, Context};
use std::collections::HashMap;

use pulldown_cmark::html::push_html;
use pulldown_cmark::Parser;
pub struct Writer {
    tera: Tera,
    input: String,
    output: String
}

impl Writer {
    pub fn new(input: String, output: String) -> Writer {
        let input_path = input.clone().add("/templates/**/*");
        let mut tera = compile_templates!(&input_path);
        tera.register_filter("convert_numeric", Self::convert_numeric);
        Writer {
            tera,
            input,
            output
        }
    }

    pub fn output(&self) -> PathBuf {
        PathBuf::from(&self.output)
    }

    pub fn input(&self) -> PathBuf {
        PathBuf::from(&self.input)
    }

    pub fn write(&self) {
        let pages = get_all_pages(&self.input()).expect("Unable to get pages");
        let db = DirBuilder::new();
        if !self.output().exists() {
            println!("Creating output folder");
            db.create(&self.output()).expect(&format!("Unable to create base folder: {:?}", &self.output));
        }
        if let Some(html) = self.index_html(&pages) {
            println!("Creating index.html");
            let index_path = self.output().join("index.html");
            let mut index = File::create(&index_path).expect("unable to create index.html");
            index.write_all(html.as_bytes()).expect("Unable to write html to index.html");
        }
        if let Some(html) = self.about_html() {
            let about_path = self.output().join("about");
            if !about_path.exists() {
                println!("creating about folder");
                db.create(&about_path).expect("Unable to create missing about path");
            }
            println!("Creating about/index.html");
            let mut about = File::create(&about_path.join("index.html")).expect("unable to create about/index.html");
            about.write_all(html.as_bytes()).expect("unable to write about/index.html");
        }
        if let Some(html) = self.contact_html() {
            let contact_path = self.output().join("contact");
            if !contact_path.exists() {
                println!("creating contact folder");
                db.create(&contact_path).expect("Unabel to create missing contact path");
            }
            println!("creating contact/index.html");
            let mut contact = File::create(&contact_path.join("index.html")).expect("unable to create about/index.html");
            contact.write_all(html.as_bytes()).expect("unable to write about/index.html");
        }
        println!("moving about image");
        let in_joe = self.input().join("joe.jpg");
        let out_joe = self.output().join("joe.jpg");
        let _ = copy(in_joe, out_joe).expect("Unable to copy joe.jpg");
        let in_portfolio_path = self.input().join("portfolio");
        let out_portfolio_path = self.output().join("portfolio");
        if !out_portfolio_path.exists() {
            println!("creating portfolio path");
            db.create(&out_portfolio_path).expect("Unable to create portfolio path");
        }
        for page in pages {
            let mut page_path = out_portfolio_path.join(&page.project.name);
            let mut input_page = in_portfolio_path.join(&page.project.name);
            if !page_path.exists() {
                println!("creaing folder for {:?}", &page.project.name);
                if let Err(_e) = db.create(&page_path) {
                    println!("unable to create: {:?}", &page_path);
                    continue;
                };
            }
            if let Some(html) = self.page_html(&page) {
                println!("creating index.html for {:?}", &page.project.name);
                let index_path = page_path.join("index.html");
                let mut index = File::create(&index_path).expect(&format!("Unable to create {:?}", &index_path));
                index.write_all(html.as_bytes()).expect(&format!("Unable to write html for {:?}", &index_path));
            }
            let out_img = page_path.join("img");
            let in_img = input_page.join("img");
            if !out_img.exists() {
                println!("creatig img folder for {}", &page.project.name);
                db.create(&out_img).expect(&format!("unable to create img forlder{:?}", &out_img));
            }
            println!("moving images for {:?}", &page.project.name);
            for img in page.project.images {
                let in_path = in_img.join(&img);
                let out_path = out_img.join(&img);
                if let Err(e) = copy(&in_path, &out_path) {
                    println!("Unable to copy image from {:?} to {:?}\n{:?}", &in_path, &out_path, &e);
                    continue;
                }
            }
        }
        self.copy_fonts();
    }

    fn copy_fonts(&self) {
        let in_fonts = self.input().join("fonts");
        let out_fonts = self.output().join("fonts");
        if !out_fonts.exists() {
            println!("creating output fonts folder");
            let db = DirBuilder::new();
            let _ = db.create(&out_fonts);
        }
        if let Ok(rd) = read_dir(&in_fonts) {
            for entry in rd {
                if let Ok(ent) = entry {
                    let from = &ent.path();
                    let name = &ent.file_name();
                    let to = out_fonts.join(&name);
                    println!("Copying font {:?}", &name);
                    if let Err(e) = copy(from, to) {
                        println!("failed to copy {:?}\n{:?}", &name, e);
                    }
                }
            }
        }

    }

    pub fn page_html(&self, page: &Page) -> Option<String> {
            let mut ctx = Context::new();
            ctx.add("page", page);
            ctx.add("route", "portfolio");
            match self.tera.render("page.html", &ctx) {
                Ok(html) => Some(html),
                Err(e) => {
                    println!("Error getting template html: {:?}", e);
                    None
                }
            }
    }

    pub fn index_html(&self, pages: &Vec<Page>) -> Option<String> {
        let mut ctx = Context::new();
        ctx.add("pages", pages);
        ctx.add("route", "index");
        match self.tera.render("index.html", &ctx) {
            Ok(body) => Some(body),
            Err(e) => {
                println!("Error rending html: {:?}", e);
                None
            }
        }
    }

    pub fn about_html(&self) -> Option<String> {
        let mut ctx = Context::new();
        ctx.add("route", "about");
        let mut content = String::new();
        match File::open(self.input().join("about.md")) {
            Ok(mut f) => {
                match f.read_to_string(&mut content) {
                    Ok(_size) => {
                        let html = Self::md_to_html(&content);
                        ctx.add("content", &html);
                    }, 
                    Err(e) => println!("Error reading content {:?}", e),
                }
            },
            Err(e) => println!("Error opening about content {:?}", e),
        };
        match self.tera.render("about.html", &ctx) {
            Ok(body) => Some(body),
            Err(e) => {
                println!("Error rendering html: {:?}", e);
                None
            }
        }
    }

    pub fn contact_html(&self) -> Option<String> {
        let mut ctx = Context::new();
        ctx.add("route", "contact");
        match self.tera.render("contact.html", &ctx) {
            Ok(body) => Some(body),
            Err(e) => {
                println!("Error rendering html: {:?}", e);
                None
            }
        }
    }

    fn convert_numeric(value: Value, _: HashMap<String, Value>) -> Result<Value> {
        match value {
            Value::String(mut text) => {
                let numbers = Self::numbers();
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
}