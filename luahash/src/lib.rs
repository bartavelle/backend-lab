use common;
use mlua::prelude::*;

fn hash_username(_lua: &Lua, (username, nonce): (String, String)) -> LuaResult<String> {
    let inonce: u128 = nonce
        .parse()
        .map_err(|_| mlua::Error::RuntimeError("malformed nonce".to_string()))?;
    let token = common::hash_username(&username, inonce);
    let r = common::AuthInfo { username, token };

    serde_json::to_string(&r)
        .map_err(|rr| mlua::Error::RuntimeError(format!("could not json encode: {}", rr)))
}

#[mlua::lua_module]
fn luahash(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hash_username", lua.create_function(hash_username)?)?;
    Ok(exports)
}
