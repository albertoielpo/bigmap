#![deny(clippy::all)]

use napi_derive::napi;
use std::collections::HashMap;

/// Wrapped class for Rust native HashMap<String,String>
/// With this structure is possible to allocate millions of entry
#[napi]
pub struct BigMap {
    map: HashMap<String, String>,
}

#[napi]
impl BigMap {
    #[napi(constructor)]
    pub fn new() -> Self {
        BigMap {
            map: HashMap::new(),
        }
    }

    #[napi]
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    #[napi]
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.map.contains_key(&key)
    }

    #[napi]
    pub fn clear(&mut self) -> () {        
        self.map.clear()
    }

    #[napi]
    pub fn len(& self) -> i64 {
        self.map.len() as i64
    }

}

