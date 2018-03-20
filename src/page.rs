use std::path::{PathBuf};
use std::fs::{read_dir, DirEntry};

// #[derive(Serialize, Debug)]
// pub struct Meta {
//     pub title: String,
//     pub context: String,
//     pub teammates: Vec<String>,
// }

// #[derive(Debug)]
// pub struct Page {
//     pub meta: Meta,
//     pub project: Project,
//     pub content: String
// }

#[derive(Debug, Serialize)]
pub struct Project {
    pub name: String,
    pub images: Vec<String>,
}

pub fn get_all_projects() -> Result<Vec<Project>, String> {
    let mut projects = Vec::<Project>::new();
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
                if let Ok(p) = get_project(&name) {
                    projects.push(p);
                }
            }
        }
        Ok(projects)
    } else {
        return Err(String::from("Unable to read portfolio"));
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
        Err(e) => println!("{:?}", e)
    };
    Ok(Project {
        name: String::from(name),
        images,
    })
}

// pub fn get_meta(path: PathBuf) -> Meta {

//     Meta {
//         title: String::new(),
//         context: String::new(),
//         teammates: vec!(),
//     }
// }