const HTTP_VERSION: &str = "HTTP/1.1";

pub fn ok(body: &str) -> String {
    format!("{} 200 OK\r\nContent-Type: text/plain\r\n\r\n{}", HTTP_VERSION, body).to_string()
}

pub fn bad_request(body: &str) -> String {
    format!("{} 400 Bad Request\r\nContent-Type: text/plain\r\n\r\n{}", HTTP_VERSION, body).to_string()
}

pub fn internal_server_error(body: &str) -> String {
    format!("{} 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\n{}", HTTP_VERSION, body).to_string()
}
