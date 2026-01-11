use std::sync::Mutex; // The "Lock"
use std::collections::HashMap;

#[allow(dead_code)]
pub struct PageCache {
    pub cache: Mutex<HashMap<u64, Vec<u8>>>,
    pub capacity: usize,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Mutex::new(HashMap::with_capacity(capacity)),
            capacity,
        }
    }
}