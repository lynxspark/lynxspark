use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod config;
mod storage;

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use storage::{Record, Storage};

static STORAGE: Mutex<Option<Storage>> = Mutex::new(None);

fn handle_storage<F, R>(callback: F) -> R
where
    F: FnOnce(&mut Storage) -> R,
{
    let mut storage_lock = STORAGE.lock().unwrap();
    if let Some(ref mut storage) = *storage_lock {
        callback(storage)
    } else {
        panic!("Store not initialized");
    }
}

fn handle_body_parser(request: &str) -> HashMap<String, String> {
    let content_index = request.find("\r\n\r\n").unwrap_or(0) + 4;
    let body = &request[content_index..];

    let mut parsed_form = HashMap::new();
    for pair in body.split('&') {
        let mut iter = pair.split('=');
        if let Some(key) = iter.next() {
            let value = iter.next().unwrap_or("").trim_end_matches('\0').to_string();

            match key {
                "key" | "value" | "expire" => {
                    parsed_form.insert(key.to_string(), value);
                }
                _ => {}
            }
        }
    }

    parsed_form
}

fn http_resp(value: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        value.len(),
        value
    )
}

fn handle_set_request(parsed_form_values: HashMap<String, String>) -> String {
    if let (Some(key), Some(value)) = (
        parsed_form_values.get("key"),
        parsed_form_values.get("value"),
    ) {
        let current_time = SystemTime::now();
        let mut expire_time = SystemTime::UNIX_EPOCH;

        if let Some(expire_str) = parsed_form_values.get("expire") {
            if let Ok(expire_secs) = expire_str.parse::<u64>() {
                expire_time = current_time + Duration::from_secs(expire_secs);
            }
        }

        return handle_storage(|store_manager| {
            let record = Record {
                key: key.to_string(),
                value: value.to_string(),
                exp_time: expire_time,
                last_acc_time: current_time,
            };

            store_manager.set(record.clone());
            http_resp("OK")
        });
    }

    http_resp("INVALID_REQ")
}

fn handle_get_request(parsed_form_values: HashMap<String, String>) -> String {
    if let Some(key) = parsed_form_values.get("key") {
        return handle_storage(|store_manager| {
            if let Some(record) = store_manager.get(key) {
                http_resp(&record.value)
            } else {
                http_resp("NOT_FOUND")
            }
        });
    }

    http_resp("BAD_REQUEST")
}

fn handle_del_request(parsed_form_values: HashMap<String, String>) -> String {
    if let Some(key) = parsed_form_values.get("key") {
        return handle_storage(|store_manager| {
            if store_manager.get(key).is_some() {
                store_manager.del(key);
                http_resp("OK")
            } else {
                http_resp("NOT_FOUND")
            }
        });
    }

    http_resp("INVALID_REQ")
}

fn handle_request(request: &str) -> String {
    let parsed_form_values = handle_body_parser(request);
    if request.contains("POST /set") {
        handle_set_request(parsed_form_values)
    } else if request.contains("GET /get") {
        handle_get_request(parsed_form_values)
    } else if request.contains("DELETE /del") {
        handle_del_request(parsed_form_values)
    } else {
        http_resp("INVALID_REQ")
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if let Ok(request) = String::from_utf8(buffer.to_vec()) {
        let response = handle_request(&request);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_periodic_expirer(interval: Duration) {
    thread::spawn(move || loop {
        thread::sleep(interval);
        handle_storage(|store_manager| store_manager.periodic_expirer());
    });
}

fn handle_idle_expirer(interval: Duration) {
    thread::spawn(move || loop {
        thread::sleep(interval);
        handle_storage(|store_manager| store_manager.idle_expirer());
    });
}

fn handle_snapshot_save(interval: Duration, filename: Arc<Mutex<String>>) {
    thread::spawn(move || loop {
        thread::sleep(interval);

        let filename = Arc::clone(&filename);
        handle_storage(|store_manager: &mut Storage| {
            let filename = filename.lock().unwrap();
            store_manager.save_to_file(filename.as_str());
        });
    });
}

fn handle_snapshot_load(filename: Arc<Mutex<String>>) {
    let filename = Arc::clone(&filename);
    handle_storage(|store_manager: &mut Storage| {
        let filename = filename.lock().unwrap();
        store_manager.load_from_file(filename.as_str());
    });
}

fn main() {
    let config = config::get_global_config();

    let storage = Storage::new();
    *STORAGE.lock().unwrap() = Some(storage);

    let filename = Arc::new(Mutex::new(config.snapshot_file_path.clone()));
    handle_snapshot_load(Arc::clone(&filename));
    handle_snapshot_save(Duration::from_secs(10), filename);

    if config.periodic_timeout_check_enable {
        handle_periodic_expirer(Duration::from_secs(config.periodic_scan_range_interval));
    }

    if config.idle_timeout_check_enable {
        handle_idle_expirer(Duration::from_secs(config.idle_scan_range_interval));
    }

    let listener = TcpListener::bind(format!("{}:{}", config.bind, config.port)).unwrap();
    println!("Server listening on port {}...", config.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
