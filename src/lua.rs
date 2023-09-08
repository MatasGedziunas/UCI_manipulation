use mlua::{Lua, Function, Value};
use std::path::Path;
use std::fs;
use std::io;
struct LuaContext{
    lua : Lua,
}
impl LuaContext{

    // Function to read Lua script content from a file
    fn read_script_from_file(file_path: &str) -> io::Result<String>{
        let script_content = fs::read_to_string(file_path)?;    
        Ok(script_content)
    }
    
    fn new() -> Result<Lua, mlua::Error>
    {
        let mut lua = Lua::new();
        // lua.load("uci_manipulation/src/helper.lua").exec().expect("Error Loading lua script");
        return Ok((lua));
    }
    
    fn call_function_in_lua(&self, function_name: &str, args: Vec<Value>) -> Result<Value, mlua::Error> 
    {
        let globals = self.lua.globals();
        let function: Function = globals.get(function_name)?;
        let ans: Value = function.call::<_, Value>(args)?;
        return Ok(ans);
    }
    
}