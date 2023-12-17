use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use super::methods;

const BUFFER_SIZE: usize = 1024;

pub struct Handler {
    pub listener: TcpListener,
    pub router: Router,
}

impl Handler {
    pub fn new(bind: &str, port: u16) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", bind, port)).expect("Failed to bind TcpListener");
        println!("LynxSpark listening on port {}...", port);

        let router = Router::new();

        Handler { listener, router }
    }

    pub fn handle_client(mut stream: TcpStream, router: &Router) {
        let mut buffer = [0; BUFFER_SIZE];
        if let Ok(_) = stream.read(&mut buffer) {
            let request = String::from_utf8_lossy(&buffer[..]);
            
            // Assuming you have a function to extract the HTTP method from the request
            if let Some(method) = methods::extract_method(&request) {
                let response = router.handle_request(&method, &request);

                if let Err(e) = stream.write(response.as_bytes()) {
                    eprintln!("Error writing to stream: {}", e);
                }

                if let Err(e) = stream.flush() {
                    eprintln!("Error flushing stream: {}", e);
                }
            } else {
                eprintln!("Error extracting HTTP method from request");
            }
        } else {
            eprintln!("Error reading from stream");
        }
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = self.router.clone();
                    let listener_clone = self
                        .listener
                        .try_clone()
                        .expect("Failed to clone TcpListener");
                    thread::spawn(move || {
                        let server = Handler {
                            listener: listener_clone,
                            router,
                        };
                        Handler::handle_client(stream, &server.router);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

impl Clone for Handler {
    fn clone(&self) -> Self {
        let bind = self.listener.local_addr().unwrap().ip().to_string();
        let port = self.listener.local_addr().unwrap().port();

        let listener =
            TcpListener::bind(format!("{}:{}", bind, port)).expect("Failed to bind TcpListener");
        let router = self.router.clone();

        Handler { listener, router }
    }
}

#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn route(&mut self, method: methods::Method, path: &str, handler: fn(request: &str) -> String) {
        let route = Route {
            method,
            path: path.to_string(),
            handler,
        };

        self.routes.push(route);
    }

    pub fn handle_request(&self, method: &methods::Method, request: &str) -> String {
        for route in &self.routes {
            if route.matches_request(method, request) {
                return (route.handler)(request);
            }
        }

        "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string()
    }
}

#[derive(Clone)]
pub struct Route {
    pub method: methods::Method,
    pub path: String,
    pub handler: fn(request: &str) -> String,
}

impl Route {
    pub fn matches_request(&self, method: &methods::Method, request: &str) -> bool {
        &self.method == method && request.contains(&self.path)
    }
}
