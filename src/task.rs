use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::fs;


pub fn list_config_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    if (dir.is_dir()) {
        let mut config_file_paths: Vec<PathBuf> = Vec::new();
        for file in fs::read_dir(dir)? {
            let file: fs::DirEntry = file?;
            if (is_config_file(file.path())) {
                config_file_paths.push(file.path());
            }
        }
        return Ok(config_file_paths);
    } else {
        println!("{}", "aaaaa");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid directory",
        ));
    }
}

fn is_config_file(file_path: PathBuf) -> bool {
    return file_path.to_str().unwrap().ends_with(".config");
}

fn get_config_file(file: &Path) -> Result<fs::File, Error> {
    if (file.is_file()) {
        let file: fs::File = fs::File::open(file)?;
        return Ok(file);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file path",
        ));
    }
}

pub fn get_config_file_content(file: &Path) -> Result<String, Error> {
    let mut file_content: String = String::new();
    let mut file: fs::File = get_config_file(file)?;
    file.read_to_string(&mut file_content)?;
    return Ok(file_content);
}