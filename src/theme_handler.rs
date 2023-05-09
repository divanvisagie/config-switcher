use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_path() -> Option<PathBuf> {
    let path = if cfg!(target_os = "macos") {
        dirs::home_dir().map(|path| path.join(".config"))
    } else {
        dirs::config_dir()
    };

    if let Some(path) = path {
        let path = path.join("alacritty");
        Some(path)
    } else {
        None
    }
}

pub fn find_yaml_files_in_dir(path: &PathBuf) -> Result<Vec<Theme>, Box<dyn Error>> {
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

    let themes = files
        .iter()
        .map(|path| Theme {
            name: get_name_from_path(path).unwrap(),
            path: path.clone(),
        })
        .collect::<Vec<_>>();

    Ok(themes)
}

fn get_name_from_path(path: &PathBuf) -> Option<String> {
    path.file_stem()
        .and_then(|name| name.to_str().map(|s| s.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(path.is_some());
    }

    #[test]
    fn test_find_yaml_files_in_dir() {
        let path = get_path().unwrap().join("themes").join("themes");
        let files = find_yaml_files_in_dir(&path);
        assert!(files.is_ok());
    }

    #[test]
    fn test_get_name_from_path() {
        let path = get_path()
            .unwrap()
            .join("themes")
            .join("themes")
            .join("gruvbox_dark.yaml");
        print!("{:?}", path);
        let name = get_name_from_path(&path);
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "gruvbox_dark");
    }
}
