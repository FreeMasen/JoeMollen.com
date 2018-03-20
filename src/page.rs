use std::path::{PathBuf};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use toml::from_str;
use markdown::file_to_html;


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

#[derive(Serialize, Deserialize, Debug)]
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
    if let Ok(c) = file_to_html(&path) {
        ret.content = c;
    } else {
        return None
    }
    if let Ok(proj) = get_project(name) {
        ret.project = proj
    } else {
        return None
    }
    if let Some(meta) = get_meta(name) {
        ret.meta = meta
    } else {
        return None
    }
    Some(ret)
}

// pub fn get_all_projects() -> Result<Vec<Project>, String> {
//     let mut projects = Vec::<Project>::new();
//     let path = PathBuf::from("portfolio");
//     if let Ok(rd) = read_dir(path) {
//         for entry in rd {
//             if let Ok(ent) = entry {
//                 if let Ok(md) = ent.metadata() {
//                     if md.is_file() {
//                         continue
//                     }
//                 }
//                 let name = name_for_entry(ent);
//                 if let Ok(p) = get_project(&name) {
//                     projects.push(p);
//                 }
//             }
//         }
//         Ok(projects)
//     } else {
//         return Err(String::from("Unable to read portfolio"));
//     }
// }

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
    Ok(Project {
        name: String::from(name),
        images,
    })
}

pub fn get_meta(name: &str) -> Option<Meta> {
    let mut path = PathBuf::from("portfolio");
    path.push(&name);
    path.push("meta.toml");
    if let Ok(mut f) = File::open(path) {
        let mut buf = String::new();
        if let Ok(_size) = f.read_to_string(&mut buf) {
            if let Ok(meta) = from_str(&buf) {
                Some(meta)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}