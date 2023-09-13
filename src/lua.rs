use mlua::{Lua, Function};
pub struct LuaContext{

}
impl LuaContext{

    // // Function to read Lua script content from a file
    // fn read_script_from_file(file_path: &str) -> io::Result<String>{
    //     let script_content = fs::read_to_string(file_path)?;    
    //     Ok(script_content)
    // }
    
    fn new() -> Result<Lua, mlua::Error>
    {
        let lua = Lua::new();
        lua.load(include_str!("helper.lua")).exec().unwrap();
        return Ok((lua));
    }
    
    pub fn call_function_in_lua(function_name: &str, args: Vec<String>) -> Result<(), mlua::Error> 
    {
        let lua = Self::new().unwrap();
        let globals = lua.globals();
        let function: Function = globals.get(function_name)?;
        let _ = function.call(args)?;
        return Ok(());
    }
}