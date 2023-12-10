use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct Record {
    pub key: String,
    pub value: String,
    pub exp_time: SystemTime,
    pub last_acc_time: SystemTime,
}

pub struct Storage {
    pub records: HashMap<String, Record>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            records: HashMap::new(),
        }
    }

    pub fn set(&mut self, entry: Record) {
        self.records.insert(entry.key.clone(), entry);
    }

    pub fn get(&mut self, key: &str) -> Option<&Record> {
        if let Some(record) = self.records.get(key) {
            // Update last_acc_time for the accessed record
            let current_time = SystemTime::now();

            if record.exp_time != SystemTime::UNIX_EPOCH {
                // Check if exp_time is in the past
                if let Ok(duration) = current_time.duration_since(record.exp_time) {
                    // If duration is negative, it means last_acc_time is in the past
                    if duration.as_secs() > 0 {
                        self.del(key);
                        return None; // Data is accessed before last_acc_time, return None
                    }
                }
            }

            let mut updated_record = record.clone();
            updated_record.last_acc_time = current_time;

            // Update the record in the hashmap
            self.records
                .insert(updated_record.key.clone(), updated_record);

            // Return a reference to the updated record
            return self.records.get(key);
        }

        None
    }

    pub fn del(&mut self, key: &str) -> Option<Record> {
        self.records.remove(key)
    }

    // Periodic expiration for all records
    pub fn periodic_expirer(&mut self) {
        let current_time = SystemTime::now();
        let keys_to_remove: Vec<String> = self
            .records
            .iter()
            .filter_map(|(key, record)| {
                // expire_time 0 değilse kontrolü ekle
                if record.exp_time != SystemTime::UNIX_EPOCH {
                    if let Ok(duration) = current_time.duration_since(record.exp_time) {
                        if duration.as_secs() > 0 {
                            Some(key.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for key in keys_to_remove {
            self.del(&key);
        }
    }

    pub fn idle_expirer(&mut self) {
        let current_time = SystemTime::now();
        let keys_to_remove: Vec<String> = self
            .records
            .iter()
            .filter_map(|(key, record)| {
                // expire_time 0 değilse kontrolü ekle
                if record.exp_time != SystemTime::UNIX_EPOCH {
                    if let (Ok(duration), Ok(exp_duration)) = (
                        current_time.duration_since(record.last_acc_time),
                        current_time.duration_since(record.exp_time),
                    ) {
                        if duration >= exp_duration {
                            Some(key.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for key in keys_to_remove {
            self.del(&key);
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split('\0').collect();

                    if parts.len() == 4 {
                        let key = parts[0].to_string();
                        let value = parts[1].to_string();
                        let exp_time =
                            SystemTime::UNIX_EPOCH + Duration::from_secs(parts[2].parse().unwrap());
                        let last_acc_time =
                            SystemTime::UNIX_EPOCH + Duration::from_secs(parts[3].parse().unwrap());

                        let record = Record {
                            key,
                            value,
                            exp_time,
                            last_acc_time,
                        };

                        self.set(record);
                    }
                }
            }
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
        {
            let mut writer = BufWriter::new(&mut file);

            for record in self.records.values() {
                let line = format!(
                    "{}\0{}\0{}\0{}\n",
                    record.key,
                    record.value,
                    record
                        .exp_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    record
                        .last_acc_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                writer
                    .write_all(line.as_bytes())
                    .expect("Write to file failed");
            }
        }
    }
}