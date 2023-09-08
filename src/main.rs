use rust_uci::Uci;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::fs;

mod args;
use args::CLIargs;
use args::Command;
use clap::Parser;

fn get_section_content(mut uci: Uci, section: &str) -> Result<String, rust_uci::error::Error>{
    return uci.get(section);
}

fn set_section (mut uci: Uci, section: &str, value: &str) -> Result<(), rust_uci::error::Error>{
    return uci.set(&section, value);
}

fn delete_section(mut uci: Uci, section: &str) -> Result<(), rust_uci::error::Error>{
    return uci.delete(&section);
}

fn list_config_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
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

fn get_config_file_content(file: &Path) -> Result<String, Error> {
    let mut file_content: String = String::new();
    let mut file: fs::File = get_config_file(file)?;
    file.read_to_string(&mut file_content)?;
    return Ok(file_content);
}

fn main() {
    let args = CLIargs::parse();
    
    let uci: Uci = Uci::new().unwrap();
    let config_dir: &Path = Path::new("/etc/config");
    
    match args.command {
        Command::ListConfigFiles => {
            let config_file_paths = list_config_files(config_dir);
            match config_file_paths {
                Ok(paths) => {
                    for path in paths {
                        println!("{}", path.display());
                    }
                }
                Err(err) => {
                    panic!("{}", err)
                }
            }
        }
        Command::PrintConfigFile { file_path } => {
            let path = std::path::Path::new(&file_path);
            let file_content: Result<String, Error> = get_config_file_content(path);
            match file_content {
                Ok(content) => println!("{}", content),
                Err(err) => panic!("{}", err),
            }
        }
        Command::PrintSection { section } => {
            match get_section_content(uci, &section){
                Ok(section) => println!("{}", section),
                Err(err) => panic!("{}", err),
            }
        }
        Command::SetSection { section, value } => {
            match set_section(uci, &section,  &value) {
                Ok(()) => println!("Section value set"),
                Err(err) => panic!("{}", err),
            }
        }
        Command::DeleteSection { section } => {
            match delete_section(uci, &section){
                Ok(()) => println!("Section deleted"),
                Err(err) => panic!("{}", err),
            }
        }
    }
}
