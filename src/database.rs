use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

const SETTINGS_DIR_ID: &str = "dir";
const SETTINGS_DBFILENAME_ID: &str = "dbfilename";

const PATTERN_ALL: &str = "*";

mod config;
mod rdb;

#[derive(Debug)]
pub struct Entry {
    value: String,
    expires_at: Option<u128>,
}

pub struct Database {
    data: HashMap<String, Entry>,
    config: HashMap<String, String>,
    metadata: HashMap<String, String>,
}

// Init related
impl Database {
    pub fn new() -> Self {
        Database {
            data: HashMap::new(),
            config: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Database {
    pub fn set(
        &mut self,
        key: String,
        value: String,
        expires_at: Option<u128>,
    ) -> anyhow::Result<()> {
        self.data.insert(key, Entry { value, expires_at });
        Ok(())
    }

    pub fn get(&mut self, key: String) -> anyhow::Result<Option<String>> {
        let Some(item) = self.data.get(&key) else {
            return Ok(None);
        };

        let value = Some(item.value.clone());
        let Some(expires_at) = item.expires_at else {
            return Ok(value);
        };

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        if now <= expires_at {
            return Ok(value);
        }

        self.delete(key)?;
        Ok(None)
    }

    pub fn delete(&mut self, key: String) -> anyhow::Result<()> {
        self.data.remove(&key);
        Ok(())
    }

    pub fn keys(&self, pattern: String) -> anyhow::Result<Vec<String>> {
        let keys = self.data.keys().cloned().collect();
        if pattern == PATTERN_ALL {
            return Ok(keys);
        }

        let filtered_keys = keys
            .iter()
            .filter(|key| key.contains(&pattern))
            .cloned()
            .collect();
        Ok(filtered_keys)
    }
}
