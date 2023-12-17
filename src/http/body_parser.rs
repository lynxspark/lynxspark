use std::collections::HashMap;

pub fn key_parser(request: &str, allowed_keys: &[&str]) -> HashMap<String, String> {
    let content_index = request.find("\r\n\r\n").unwrap_or(0) + 4;
    let body = &request[content_index..];

    let mut parsed_form = HashMap::new();
    for pair in body.split('&') {
        let mut iter = pair.split('=');

        if let Some(key) = iter.next() {
            let value = iter.next().unwrap_or("").trim_end_matches('\0').to_string();

            if allowed_keys.contains(&key) {
                parsed_form.insert(key.to_string(), value);
            }
        }
    }

    parsed_form
}