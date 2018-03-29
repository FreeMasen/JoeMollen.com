
#[macro_use]
extern crate tera;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate percent_encoding;
extern crate pulldown_cmark;
extern crate clap;

use std::io::{Write};
use std::fs::{File, DirBuilder, read_dir};
use std::path::PathBuf;

use clap::{App, Arg, SubCommand};
mod page;
mod writer;

fn main() {
    let app =  App::new("JoeMollen.com Generator")
        .author("Robert Masen <r.f.masen@gmail.com>")
        .about("Generates the static html for JoeMollen.com")
        .usage("jm [SUBCOMMAND] [OPTIONS]")
        .subcommand(
            SubCommand::with_name("build")
                .about("Generate your site")
                .arg(Arg::with_name("input")
                    .takes_value(true)
                    .short("i")
                    .help("The path to the input folder, default: ./input"))
                .arg(Arg::with_name("output")
                    .takes_value(true)
                    .short("o")
                    .help("the path where the files should be written, default: ./www"))
        )
        .subcommand(
            SubCommand::with_name("layout")
                .about("Display the required input layout tree"))
                .arg(Arg::with_name("input")
                    .takes_value(true)
                    .short("i")
                    .help("The path to the input folder, default: ./input"))
                .arg(Arg::with_name("output")
                    .takes_value(true)
                    .short("o")
                    .help("the path where the files should be written, default: ./www\nNOTE: Only used for build subcommand"))
        .subcommand(
            SubCommand::with_name("setup")
                .about("Setup the current folder with the required input folder and one dummy project")
                .arg(Arg::with_name("input")
                    .takes_value(true)
                    .short("i")
                    .help("The path to the input folder, default: ./input"))
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new empty project folder to the portfolio directory")
                .arg(Arg::with_name("input")
                    .takes_value(true)
                    .short("i")
                    .help("The path to the input folder, default: ./input"))
        );
        let mut help = app.clone();
        let matches = app.get_matches();
    if let Some(_matches) = matches.subcommand_matches("layout") {
        print_layout();
    } else if let Some(matches) = matches.subcommand_matches("setup") {
        let input = matches.value_of("input").unwrap_or("input");
        setup_input(input);
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let input = matches.value_of("input").unwrap_or("input");
        new_project(input);
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let input = matches.value_of("input").unwrap_or("input");
        let output = matches.value_of("output").unwrap_or("www");
        println!("writing files from {:?} to {:?}", input, output);
        let w = ::writer::Writer::new(String::from(input), String::from(output));
        w.write();
    } else {
        let _ = help.print_help();
    }
}

fn print_layout() {
    println!("input");
    println!("----------");
    println!("/");
    println!("├─ portfolio");
    println!("│     └─ [project name] (repeated)");
    println!("│           ├─ img ");
    println!("│           │   └─ [image] (repeated)");
    println!("│           ├─ content.md");
    println!("│           └─ meta.toml");
    println!("├─ about.md");
    println!("└─ joe.jpg [image file]");
    println!("");
    println!("output");
    println!("----------");
    println!("/");
    println!("├─ portfolio");
    println!("│     └─ [project name] (repeated)");
    println!("│           ├─ img ");
    println!("│           │   └─ [image] (repeated)");
    println!("│           ├─ index.html");
    println!("├─ about");
    println!("│     └─index.html");
    println!("├─ contact");
    println!("│     └─index.html");
    println!("└─ index.html");
}

fn new_project(input: &str) {
    println!("Creating a new project folder");
    let path = PathBuf::from(&input);
    let mut db = DirBuilder::new();
    db.recursive(true);
    let port_path = path.join("portfolio");
    let count = if let Ok(rd) = read_dir(&port_path) {
        rd.count()
    } else {
        0
    };
    let project_path = port_path.join(&format!("project-{}", count));
    let img_path = project_path.join("img");
    if let Err(e) = db.create(&img_path) {
        println!("Unable to create {:?}\n{:?}", &img_path, e);
    }
    let cont_path = project_path.join("content.md");
    if let Err(e) = File::create(&cont_path) {
        println!("Unable to create {:?}\n{:?}", &cont_path, e);
    }
    let meta_path = project_path.join("meta.toml");
    if let Ok(mut f) = File::create(&meta_path) {
        let content = "title = \"New Project\"\ncontext = \"from somewhere\"\nteammates = []";
        if let Err(e) = f.write_all(content.as_bytes()) {
            println!("Unable to write the contents of meta.toml {:?}", e);
        }
    } else {
        println!("Unable to create {:?}", &meta_path)
    }
}

fn setup_input(input: &str) {
    println!("Setting up input folder {}", &input);
    let path = PathBuf::from(&input);
    let mut db = DirBuilder::new();
    db.recursive(true);
    println!("Creating a fonts folder");
    let fonts_path = path.join("fonts");
    if let Err(e) = db.create(&fonts_path) {
        println!("Unable to create fonts folder in {:?}\n{:?}", &fonts_path, e);
    }
    println!("Creating a portfolio folder");
    let port_path = path.join("portfolio");
    if let Err(e) = db.create(&port_path) {
        println!("Unable to create portfolio folder in {:?}\n{:?}", &port_path, e);
    }
    println!("Creating an about.md file");
    let about_path = path.join("about.md");
    if let Err(e) = File::create(&about_path) {
        println!("Unable to create about.md file in {:?}\n{:?}", &about_path, e);
    }
    new_project(input);
}