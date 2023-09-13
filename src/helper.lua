require("uci")
x = uci.cursor()
config = "config"

local function print_callback(s)
    print("-------------------")
    for key, value in pairs(s) do 
        print(key .. ': ' .. tostring(value))
    end
end

function get_section_content(...)
    local section = select(1, ...)
    print(config, section)
    x:foreach(config, section, print_callback)
end

function set_section(...)
    local section, type = select(1, ...), select(2, ...)
    x:set(config, section, type)
    x:commit(config)
end

function delete_section(...)
    local section = select(1, ...)
    x:delete(config, section)
    x:commit(config)
end

function set_option_value(...)
    local section, option, value = select(1, ...), select(2, ...), select(3, ...)
    x:set(config, section, option, value)
    x:commit(config)
end

function delete_option(...)
    local section, option = select(1, ...), select(2, ...)
    x:delete(config, section, option)
    x:commit(config)
end

print(get_section_content("general"))