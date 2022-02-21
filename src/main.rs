#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::Path;
use walkdir::WalkDir;
use std::fs;

fn main() {
    for entry in WalkDir::new("./data")
        .into_iter()
        .filter(|dir| match dir {
            Ok(dir_found) => {
                dir_found.path().parent()
                .and_then(Path::to_str)
                .map_or(false, |s| s.contains("locales"))
            }
            Err(_) => false,
        })
        .flatten()
    {
        let d_name = entry.path().parent().and_then(Path::to_str);
        let f_name = entry.file_name().to_string_lossy();
        if let Some(path) = d_name {
            if path.contains("locales") && f_name.ends_with(".json") {
                println!("{:?}", fs::read_to_string(entry.path()).unwrap_or_default());
            }
            
        }
    }
}

// WHY NOT USE GLOB YOU FOOL ????????
// use glob::glob;

// fn main() -> Result<()> {
//    for entry in glob("**/*.png")? {
//        println!("{}", entry?.display());
//    }

//    Ok(())
//}
