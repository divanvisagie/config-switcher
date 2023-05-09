mod theme_handler;

use theme_handler::{find_yaml_files_in_dir, get_path};

fn main() {
    let config_root = get_path();
    println!("config {:?}", config_root);
    let theme_directory = config_root.unwrap().join("themes").join("themes");
    match find_yaml_files_in_dir(&theme_directory) {
        Ok(themes) => {
            for theme in themes {
                println!("{:?}", theme);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
