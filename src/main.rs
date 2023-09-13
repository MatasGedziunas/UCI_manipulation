use std::path::Path;
use std::io::Error;

mod task;
mod args;
mod lua;

use args::CLIargs;
use args::Command;
use clap::Parser;
use task::*;



fn main() {
    let args = CLIargs::parse();
    
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
            match get_section_content(&section){
                Ok(()) => (),
                Err(err) => panic!("{}", err),
            }
        }
        Command::SetSection { section, type_of_section } => {
            match set_section(&section, &type_of_section) {
                Ok(()) => println!("Section value set"),
                Err(err) => panic!("{}", err),
            }
        }
        Command::DeleteSection { section } => {
            match delete_section(&section){
                Ok(()) => println!("Section deleted"),
                Err(err) => panic!("{}", err),
            }
        }
        Command::SetOptionValue { section, option, value } => {
            match set_option_value(&section, &option, &value) {
                Ok(()) => println!("Option value set"),
                Err(err) => panic!("{}", err),
            }
        }
        Command::DeleteOption {section, option } => {
            match delete_option(&section, &option) {
                Ok(()) => println!("Section deleted"),
                Err(err) => panic!("{}", err),
            }
        }
    }
}
