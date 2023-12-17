use crate::{
    http::{self, body_parser::key_parser},
    storage, utils,
};

pub fn path() -> &'static str {
    "/keys"
}

pub fn command(request: &str) -> String {
    let allowed_keys = ["pattern"];
    let keys = key_parser(request, &allowed_keys);

    if let Some(pattern) = keys.get("pattern") {
        match storage::manager::keys(&utils::url::decode(pattern)) {
            Ok(results) => {
                if !results.is_empty() {
                    let mut formatted_results = String::new();

                    for (index, result) in results.iter().enumerate() {
                        formatted_results.push_str(&format!(r#"{}) "{}""#, index + 1, result));

                        if index < results.len() - 1 {
                            formatted_results.push_str("\n");
                        }
                    }

                    http::response::ok(formatted_results.as_str())
                } else {
                    http::response::ok("No matching keys found.")
                }
            }
            Err(err) => http::response::internal_server_error(err.as_str()),
        }
    } else {
        http::response::bad_request("Missing key parameter")
    }
}
