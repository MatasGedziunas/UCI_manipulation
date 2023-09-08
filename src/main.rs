use rust_uci::Uci;
use std::path::Path;
use std::io::Error;
mod task;
mod args;
mod lua;

use args::CLIargs;
use args::Command;
use clap::Parser;



fn main() {
    let args = CLIargs::parse();
    
    let uci: Uci = Uci::new().unwrap();
    let config_dir: &Path = Path::new("/etc/config");
    
    match args.command {
        Command::ListConfigFiles => {
            let config_file_paths = task::list_config_files(config_dir);
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
            let file_content: Result<String, Error> = task::get_config_file_content(path);
            match file_content {
                Ok(content) => println!("{}", content),
                Err(err) => panic!("{}", err),
            }
        }
        Command::PrintSection { section } => {
            // match get_section_content(uci, &section){
            //     Ok(section) => println!("{}", section),
            //     Err(err) => panic!("{}", err),
            // }
        }
        Command::SetSection { section, value } => {
            // match set_section(uci, &section,  &value) {
            //     Ok(()) => println!("Section value set"),
            //     Err(err) => panic!("{}", err),
            // }
        }
        Command::DeleteSection { section } => {
            // match delete_section(uci, &section){
            //     Ok(()) => println!("Section deleted"),
            //     Err(err) => panic!("{}", err),
            // }
        }
    }
}
