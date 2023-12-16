#[derive(Clone, PartialEq)] // Add PartialEq to derive for enum
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

pub fn extract_method(request: &str) -> Option<Method> {
    // Assuming the HTTP method is the first word in the request
    let method_str = request.split_whitespace().next()?;
    
    match method_str {
        "GET" => Some(Method::GET),
        "POST" => Some(Method::POST),
        "PUT" => Some(Method::PUT),
        "DELETE" => Some(Method::DELETE),
        "HEAD" => Some(Method::HEAD),
        "OPTIONS" => Some(Method::OPTIONS),
        "PATCH" => Some(Method::PATCH),
        "TRACE" => Some(Method::TRACE),
        "CONNECT" => Some(Method::CONNECT),
        // Add other HTTP methods as needed
        _ => None,
    }
}