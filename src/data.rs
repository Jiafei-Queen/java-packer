use std::collections::HashMap;

pub struct Config {
    pub prefix: String,
    pub key: String,
}

impl Config {
    pub fn from_twin_vec(prefix: Vec<&str>, keys: Vec<&str>) -> Result<Vec<Self>, String> {
        if prefix.is_empty() || keys.is_empty() {
            return Err("prefix and keys must not be empty".to_string());
        }

        if prefix.len() != keys.len() {
            return Err("prefix and keys must have the same length".to_string());
        }

        let mut vec: Vec<Self> = Vec::new();
        for i in 0..keys.len() {
            vec.push(Self { prefix: prefix[i].to_string(), key: keys[i].to_string() });
        }

        Ok(vec)
    }
}

// Key & Value
pub type Set = HashMap<String, String>;

// Section & Set
pub type Toml = HashMap<String, Set>;