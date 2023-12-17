use crate::{
    http::{self, body_parser::key_parser}, 
    storage,
};

pub fn path() -> &'static str {
    "/set"
}

pub fn command(request: &str) -> String {
    let allowed_keys = ["key", "value"];
    let keys = key_parser(request, &allowed_keys);

    if keys.get("key").is_none() {
        return http::response::bad_request("Missing key parameter");
    }

    if keys.get("value").is_none() {
        return http::response::bad_request("Missing value parameter");
    }
    
    let key = keys.get("key").unwrap();
    let value = keys.get("value").unwrap();
    
    // Handle the result of add_item
    if let Err(err) = storage::manager::add_item(&key, &value) {
        // Return an error response
        return http::response::internal_server_error(&format!("Error adding item to storage: {}", err));
    }

    // Return an OK response
    http::response::ok("OK")
}
