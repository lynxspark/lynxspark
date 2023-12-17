use crate::{
    http::{self, body_parser::key_parser}, 
    storage,
};

pub fn path() -> &'static str {
    "/del"
}

pub fn command(request: &str) -> String {
    let allowed_keys = ["key"];
    let keys = key_parser(request, &allowed_keys);

    if keys.get("key").is_none() {
        return http::response::bad_request("Missing key parameter");
    }
    
    let key = keys.get("key").unwrap();
    // Handle the result of add_item
    if let Err(err) = storage::manager::del(&key) {
        // Return an error response
        return http::response::internal_server_error(&format!("Error adding item to storage: {}", err));
    }

    // Return an OK response
    http::response::ok("OK")
}
