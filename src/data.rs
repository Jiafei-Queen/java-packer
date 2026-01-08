use std::collections::HashMap;

// Key & Value
pub type Set = HashMap<String, String>;

// Section & Set
pub type Toml = HashMap<String, Set>;

pub enum OS { Unix, Windows }