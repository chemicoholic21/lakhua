use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

use log::debug;
use once_cell::sync::Lazy;

use crate::config::{self, SUPPORTED_RESOLUTIONS};
use crate::error::LakhuaError;
use crate::types::LocationDetails;

type Store = HashMap<String, LocationDetails>;

static STORES: Lazy<RwLock<HashMap<u8, Store>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data")
}

pub fn read_reverse_geo_store(resolution: u8) -> Result<Store, LakhuaError> {
    let path = data_dir().join(format!(
        "{}{}.json",
        config::DATA_FILE_PREFIX, resolution
    ));

    if !path.exists() {
        return Err(LakhuaError::DataFileNotFound(resolution));
    }

    debug!("Loading data file: {}", path.display());
    let contents = fs::read_to_string(&path)?;
    let store: Store = serde_json::from_str(&contents)?;
    debug!(
        "Loaded {} entries for resolution {}",
        store.len(),
        resolution
    );

    Ok(store)
}

pub fn load_all_stores_once() -> Result<(), LakhuaError> {
    {
        let stores = STORES.read().expect("store lock poisoned");
        if !stores.is_empty() {
            return Ok(());
        }
    }

    let mut stores = STORES.write().expect("store lock poisoned");
    if !stores.is_empty() {
        return Ok(());
    }

    for &resolution in &SUPPORTED_RESOLUTIONS {
        let store = read_reverse_geo_store(resolution)?;
        stores.insert(resolution, store);
    }

    debug!("All stores loaded");
    Ok(())
}

pub fn get_store(resolution: u8) -> Option<Store> {
    let stores = STORES.read().expect("store lock poisoned");
    stores.get(&resolution).cloned()
}

#[allow(dead_code)]
pub fn set_stores_for_testing(test_stores: HashMap<u8, Store>) {
    let mut stores = STORES.write().expect("store lock poisoned");
    *stores = test_stores;
}

#[allow(dead_code)]
pub fn clear_store_cache() {
    let mut stores = STORES.write().expect("store lock poisoned");
    stores.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn test_read_reverse_geo_store_resolution_5() {
        let _lock = lock();
        clear_store_cache();

        let store = read_reverse_geo_store(5).unwrap();
        assert!(!store.is_empty());

        let entry = store.get("853d838bfffffff").unwrap();
        assert_eq!(entry.city, "Orchha");
        assert_eq!(entry.state, "Madhya Pradesh");
    }

    #[test]
    fn test_read_reverse_geo_store_resolution_4() {
        let _lock = lock();
        clear_store_cache();

        let store = read_reverse_geo_store(4).unwrap();
        assert!(!store.is_empty());

        let entry = store.get("843d839ffffffff").unwrap();
        assert_eq!(entry.city, "Orchha");
        assert_eq!(entry.state, "Madhya Pradesh");
    }

    #[test]
    fn test_read_reverse_geo_store_invalid_resolution() {
        let _lock = lock();
        clear_store_cache();

        let result = read_reverse_geo_store(99);
        assert!(result.is_err());
    }
}
