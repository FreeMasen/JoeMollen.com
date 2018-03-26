use std::path::{PathBuf};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use toml::from_str;
use writer::Writer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}

impl Meta {
    pub fn default() -> Meta {
        Meta {
            title: String::new(),
            context: String::new(),
            teammates: vec!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub meta: Meta,
    pub project: Project,
    pub content: String
}

impl Page {
    pub fn default() -> Page {
        Page {
            meta: Meta::default(),
            project: Project::default(),
            content: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub images: Vec<String>,
}

impl Project {
    pub fn default() -> Project {
        Project {
            name: String::new(),
            images: vec!()
        }
    }
}

pub fn get_all_pages(base_path: &PathBuf) -> Option<Vec<Page>> {
    let path = base_path.join("portfolio");
    let mut pages: Vec<Page> = vec!();
    if let Ok(rd) = read_dir(&path) {
        for entry in rd {
            if let Ok(ent) = entry {
                if let Ok(md) = ent.metadata() {
                    if md.is_file() {
                        continue
                    }
                }
                let name = name_for_entry(ent);
                let page_path = &path.join(name);
                if let Some(p) = get_page(&page_path) {
                    pages.push(p);
                }
            }
        }
    }
    Some(pages)
}

pub fn get_page(path: &PathBuf) -> Option<Page> {
    let mut ret = Page::default();
    match File::open(&path.join("content.md")) {
        Ok(mut f) => {
            let mut buf = String::new();
            ret.content = match f.read_to_string(&mut buf) {
                Ok(_size) => Writer::md_to_html(&buf),
                Err(e) => {
                    println!("Error reading content {:?}",e);
                    String::new()
                }
            };
            ret.project = match get_project(&path) {
                Ok(p) => p,
                Err(e) => {
                    println!("Error getting project {:?}", e);
                    Project::default()
                }
            };
            ret.meta = match get_meta(&path) {
                Some(meta) => meta,
                None => {
                    println!("Unable to get meta");
                    Meta::default()
                }
            };
            Some(ret)
        },
        Err(e) => {
            println!("unable to open page content.md {:?}\n{:?}", e, path);
            None
        }
    }
}

fn name_for_entry(entry: DirEntry) -> String {
    if let Ok(n) = entry.file_name().into_string() {
        n
    } else {
        String::new()
    }
}

pub fn get_project(path: &PathBuf) -> Result<Project, String> {
    let mut images = Vec::<String>::new();
    match read_dir(path.join("img")) {
        Ok(rd) => {
            for entry in rd {
                if let Ok(ent) = entry {
                    let name = name_for_entry(ent);
                    if name.starts_with(".") {
                        continue
                    }
                    images.push(name)
                }
            }
        },
        Err(e) => println!("Error reading images: {:?}", e)
    };

    match path.file_name() {
        Some(os_str) => {
            match os_str.to_str() {
                Some(name) => {
                    Ok(Project {
                        name: String::from(name),
                        images
                    })
                },
                None => Err(String::from("unable to convert from os string to string"))
            }
        },
        None => Err(String::from("unable to get filename"))
    }
}

pub fn get_meta(path: &PathBuf) -> Option<Meta> {
    match File::open(path.join("meta.toml")) {
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Ok(_size) => {
                    match from_str(&buf) {
                        Ok(m) => Some(m),
                        Err(e) => {
                            println!("Error parsing toml {:?}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    println!("Error reading toml {:?}", e);
                    None
                }
            }
        },
        Err(e) => {
            println!("Error opening meta.toml {:?} for {:?}", e, &path);
            None
        }
    }
}