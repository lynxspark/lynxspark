
mod http;
use http::server::Handler;
use http::methods::Method;

mod commands;

fn main() {
    struct Config {
        bind: String,
        port: u16,
    }

    let config = Config {
        bind: String::from("127.0.0.1"),
        port: 8080,
    };
    
    let mut app = Handler::new(&config.bind, config.port);

    app.router.route(Method::GET, &commands::set::path(),  commands::set::command);
    app.router.route(Method::POST, &commands::get::path(),  commands::get::command);
    

    app.start();
}
