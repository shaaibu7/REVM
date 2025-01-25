pub mod storage {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct KeyValue {
        storage: HashMap<String, u64>,
    }

    impl KeyValue {
        pub fn new() -> Self {
            KeyValue { 
                storage: HashMap::new(),
            }
        }

        pub fn load(&self, key: &str) -> Option<&u64> {
            self.storage.get(key)
        }

        pub fn store(&mut self, key: &str, value: u64) {
            self.storage.insert(key.to_string(), value);
        }
    }

    #[derive(Debug)]
    pub struct Storage {
        kv_store: KeyValue,
        cache: HashSet<String>
    }

    impl Storage {
        pub fn new() -> Self {
            Storage {
                kv_store: KeyValue::new(),
                cache: HashSet::new(),
            }
        }

        pub fn load(&mut self, key: &str) -> (bool, u64) {
            let warm = self.cache.contains(key);

            if !warm {
                self.cache.insert(key.to_string());
            }

            let value = self.kv_store.load(key).unwrap_or(&0);
            (warm, *value)
        }

        pub fn store(&mut self, key: &str, value: u64) {
            self.kv_store.store(key, value);
        }
    }

}