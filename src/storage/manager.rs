use std::sync::{Once, RwLock};
use std::collections::HashMap;

static INIT: Once = Once::new();
static mut STORAGE: *mut Option<RwLock<HashMap<String, String>>> = std::ptr::null_mut();

pub fn initialize() {
    INIT.call_once(|| {
        unsafe {
            STORAGE = Box::into_raw(Box::new(Some(RwLock::new(HashMap::new()))));
        }
    });
}

pub fn add_item(key: &str, value: &str) -> Result<(), String> {
    unsafe {
        let storage = &mut *STORAGE;
        if let Some(ref mut storage) = *storage {
            let mut guard = storage.write().map_err(|e| e.to_string())?;
            guard.insert(key.to_string(), value.to_string());
            Ok(())
        } else {
            Err("Storage is not initialized.".to_string())
        }
    }
}

pub fn get_item(key: &str) -> Result<Option<String>, String> {
    unsafe {
        let storage = &*STORAGE;
        if let Some(ref storage) = *storage {
            let guard = storage.read().map_err(|e| e.to_string())?;
            Ok(guard.get(key).cloned())
        } else {
            Err("Storage is not initialized.".to_string())
        }
    }
}

pub fn delete_item(key: &str) -> Result<(), String> {
    unsafe {
        let storage = &mut *STORAGE;
        if let Some(ref mut storage) = *storage {
            let mut guard = storage.write().map_err(|e| e.to_string())?;
            guard.remove(key);
            Ok(())
        } else {
            Err("Storage is not initialized.".to_string())
        }
    }
}
