use crate::{
    http::{self, body_parser::key_parser}, storage,
};

pub fn path() -> &'static str {
    "/get"
}

pub fn command(request: &str) -> String {
    let allowed_keys = ["key"];
    let keys = key_parser(request, &allowed_keys);

    if keys.get("key").is_none() {
        return http::response::bad_request("Missing key parameter");
    }
    
    let key = keys.get("key").unwrap();
    
    if let Ok(Some(value)) = storage::manager::get_item(&key) {
        http::response::ok(value.as_str())
    } else {
        http::response::ok("NOT_FOUND")
    }
}
