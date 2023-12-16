pub fn path() -> String {
    "/set".to_string()
}

pub fn command(request: &str) -> String{
    println!("{}", request);
   "HTTP/1.1 200 OK\r\n\r\n SET".to_string()
}
