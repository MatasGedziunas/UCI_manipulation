use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::fs;

use crate::lua::LuaContext;



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

pub fn get_section_content(section: &str) -> Result<(), mlua::Error>
{
    match LuaContext::call_function_in_lua("get_section_content", vec![section.to_string()]){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn set_section(section: &str, type_of_section: &str) -> Result<(), mlua::Error> {
    match LuaContext::call_function_in_lua("set_section", vec![section.to_string(), type_of_section.to_string()]){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn delete_section(section: &str) -> Result<(), mlua::Error> {
    match LuaContext::call_function_in_lua("delete_section", vec![section.to_string()]){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn set_option_value(section: &str, option: &str, value: &str) -> Result<(), mlua::Error> {
    match LuaContext::call_function_in_lua("set_option_value", vec![section.to_string(), option.to_string(), value.to_string()]){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn delete_option(section: &str, option: &str) -> Result<(), mlua::Error> {
    match LuaContext::call_function_in_lua("delete_option", vec![section.to_string(), option.to_string()]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
