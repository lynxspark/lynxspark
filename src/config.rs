use std::fs::File;
use std::io::{self, BufReader, Read};
use std::sync::Once;

// Config yapısı
#[derive(Debug, Default)]
pub struct Config {
    pub bind: String,
    pub port: u16,
    pub snapshot_file_path: String,
    pub snapshot_scan_range_interval: u64,
    pub idle_timeout_check_enable: bool, 
    pub idle_scan_range_interval: u64,
    pub periodic_timeout_check_enable: bool,
    pub periodic_scan_range_interval: u64,
}

impl Config {
    // Config dosyasını oku ve değerleri uygula
    pub fn from_file(file_path: &str) -> Result<Self, io::Error> {
        let mut content = String::new();
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                reader.read_to_string(&mut content)?;
                let mut config: Config = Self::parse_config(&content);
                // Dosyada belirtilmeyen değerler için varsayılanları kullan
                config.apply_defaults();
                Ok(config)
            }
            Err(_) => Ok(Config::default()), // Dosya bulunamazsa varsayılan değerleri kullan
        }
    }

    // Dosyadaki içeriği parse et
    fn parse_config(content: &str) -> Config {
        let mut config = Config::default();

        for line in content.lines() {
            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();

            if parts.len() == 2 {
                match parts[0] {
                    "bind" => config.bind = String::from(parts[1]),
                    "port" => {
                        if let Ok(port) = parts[1].parse() {
                            config.port = port;
                        }
                    },
                    "snapshot_file_path" => config.snapshot_file_path = String::from(parts[1]),
                    "snapshot_scan_range_interval" => {
                        if let Ok(snapshot_scan_range_interval) = parts[1].parse() {
                            config.snapshot_scan_range_interval = snapshot_scan_range_interval;
                        }
                    },
                    "idle_timeout_check_enable" => {
                        if let Ok(idle_timeout_check_enable) = parts[1].parse() {
                            config.idle_timeout_check_enable = idle_timeout_check_enable;
                        }
                    },
                    "idle_scan_range_interval" => {
                        if let Ok(idle_scan_range_interval) = parts[1].parse() {
                            config.idle_scan_range_interval = idle_scan_range_interval;
                        }
                    },
                    "periodic_timeout_check_enable" => {
                        if let Ok(periodic_timeout_check_enable) = parts[1].parse() {
                            config.periodic_timeout_check_enable = periodic_timeout_check_enable;
                        }
                    },
                    "periodic_scan_range_interval" => {
                        if let Ok(periodic_scan_range_interval) = parts[1].parse() {
                            config.periodic_scan_range_interval = periodic_scan_range_interval;
                        }
                    }
                    _ => {} // Geçersiz bir anahtar, ignore et
                }
            }
        }

        config
    }

    // Varsayılan değerleri ayarla
    fn apply_defaults(&mut self) {
        if self.bind.is_empty() {
            self.bind = String::from("0.0.0.0");
        }
        if self.port == 0 {
            self.port = 5757;
        }
        if self.snapshot_file_path.is_empty() {
            self.snapshot_file_path = String::from("./data.lsdb");
        }
        if self.snapshot_scan_range_interval == 0 {
            self.snapshot_scan_range_interval = 10;
        }
        self.idle_timeout_check_enable = true;
        if self.idle_scan_range_interval == 0 {
            self.idle_scan_range_interval = 10;
        }
        self.periodic_timeout_check_enable = true;
        if self.periodic_scan_range_interval == 0 {
            self.periodic_scan_range_interval = 10;
        }
    }
}

// Lazy initialization için Once tipini kullan
static INIT: Once = Once::new();
static mut GLOBAL_CONFIG: Option<Config> = None;

// Global konfigürasyonu başlat
fn init_global_config() {
    INIT.call_once(|| {
        let config_file_path = "lynxspark.conf";
        match Config::from_file(config_file_path) {
            Ok(config) => unsafe {
                GLOBAL_CONFIG = Some(config);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    });
}

// Global konfigürasyona erişim sağla
pub fn get_global_config() -> &'static Config {
    unsafe {
        if GLOBAL_CONFIG.is_none() {
            init_global_config();
        }
        GLOBAL_CONFIG.as_ref().unwrap()
    }
}
