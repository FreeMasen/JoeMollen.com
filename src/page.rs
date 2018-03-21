use std::path::{PathBuf};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use toml::from_str;
use md::md_to_html;

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

pub fn get_all_pages() -> Option<Vec<Page>> {
    let mut pages: Vec<Page> = vec!();
    let path = PathBuf::from("portfolio");
    if let Ok(rd) = read_dir(path) {
        for entry in rd {
            if let Ok(ent) = entry {
                if let Ok(md) = ent.metadata() {
                    if md.is_file() {
                        continue
                    }
                }
                let name = name_for_entry(ent);
                if let Some(p) = get_page(&name) {
                    println!("Adding page: {:?}", &name);
                    pages.push(p);
                }
            }
        }
    }
    Some(pages)
}

pub fn get_page(name: &str) -> Option<Page> {
    
    let mut path = PathBuf::from("portfolio");
    let mut ret = Page::default();
    path.push(&name);
    path.push("content.md");
    match File::open(&path) {
        Ok(mut f) => {
            let mut buf = String::new();
            ret.content = match f.read_to_string(&mut buf) {
                Ok(_size) => md_to_html(&buf),
                Err(e) => {
                    println!("Error reading content {:?}",e);
                    String::new()
                }
            };
            ret.project = match get_project(name) {
                Ok(p) => p,
                Err(e) => {
                    println!("Error getting project {:?}", e);
                    Project::default()
                }
            };
            ret.meta = match get_meta(name) {
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

pub fn get_project(name: &str) -> Result<Project, String> {
    let mut images = Vec::<String>::new();
    let mut path = PathBuf::from("portfolio");
    path.push(&name);
    path.push("img");
    match read_dir(path) {
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
    if images.len() < 1 {
        images.push(String::from("empty.jpg"));
    }
    Ok(Project {
        name: String::from(name),
        images,
    })
}

pub fn get_meta(name: &str) -> Option<Meta> {
    let mut path = PathBuf::from("portfolio");
    path.push(&name);
    path.push("meta.toml");
    match File::open(path) {
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
            println!("Error opening meta.toml {:?} for {:?}", e, &name);
            None
        }
    }
}