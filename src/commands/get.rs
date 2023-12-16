pub fn path() -> String {
     "/get".to_string()
}

pub fn command(request: &str) -> String{
    println!("{}", request);
    "HTTP/1.1 200 OK\r\n\r\n GET".to_string()
}
