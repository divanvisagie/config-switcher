use std::{error::Error, path::PathBuf};

fn get_path() -> Option<PathBuf> {
    let path = dirs::config_dir();

    if let Some(path) = path {
        let path = path.join("alacritty");
        Some(path)
    } else {
        None
    }
}

fn find_yaml_files_in_dir(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yaml" {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}

fn main() {
    let config_root = get_path();
    let theme_directory = config_root.unwrap().join("themes").join("themes");

    match find_yaml_files_in_dir(&theme_directory) {
        Ok(files) => {
            for file in files {
                println!("{}", file.to_str().unwrap());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
