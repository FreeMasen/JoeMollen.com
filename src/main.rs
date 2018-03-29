
#[macro_use]
extern crate tera;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate percent_encoding;
extern crate pulldown_cmark;
extern crate clap;

use clap::{App, Arg, SubCommand};
mod page;
mod writer;

fn main() {
    let matches = App::new("JoeMollen.com Generator")
        .author("Robert Masen <r.f.masen@gmail.com>")
        .about("Generates the static html for JoeMollen.com")
        .arg(Arg::with_name("input")
            .takes_value(true)
            .short("i")
            .help("The path to the input folder, default: ./input"))
        .arg(Arg::with_name("output")
            .takes_value(true)
            .short("o")
            .help("the path where the files should be written, default: ./www"))
        .subcommand(SubCommand::with_name("layout").about("Display the required input layout tree"))
        .get_matches();
    if let Some(_sub) = matches.subcommand_matches("layout") {
        print_layout();
    } else {
        let input = matches.value_of("input").unwrap_or("input");
        let output = matches.value_of("output").unwrap_or("www");
        println!("writing files from {:?} to {:?}", input, output);
        let w = ::writer::Writer::new(String::from(input), String::from(output));
        w.write();
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