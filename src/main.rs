mod http;
pub mod storage;
use http::methods::Method;
use http::server::Handler;

mod command;

fn main() {
    storage::manager::initialize();

    struct Config {
        bind: String,
        port: u16,
    }

    let config = Config {
        bind: String::from("127.0.0.1"),
        port: 8080,
    };

    let mut app = Handler::new(&config.bind, config.port);
    app.router
        .route(Method::POST, &command::set::path(), command::set::command);
    app.router
        .route(Method::GET, &command::get::path(), command::get::command);
    app.router
        .route(Method::DELETE, &command::del::path(), command::del::command);

    app.start();
}
