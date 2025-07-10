use std::collections::HashMap;
use std::env;

use super::ConfigSource;
use crate::{ConfigValue, ParserError};

#[derive(Clone)]
pub struct CliSource {
    priority: u8,
}

impl CliSource {
    /// Creates a new `CliSource` with the given priority.
    /// The default priority is 0.
    pub fn new(priority: u8) -> Self {
        CliSource { priority }
    }

    /// Retrieves the command-line arguments.
    fn get_args() -> Vec<String> {
        env::args()
            .skip(1)
            .collect()
    }
}

impl ConfigSource for CliSource {
    fn load(&self) -> Result<ConfigValue, ParserError> {
        let mut map: HashMap<String, ConfigValue> = HashMap::new();

        // Use the get_args function to gather CLI args
        let args = Self::get_args();

        // Parse --key=val flags
        for arg in args {
            if let Some(stripped) = arg.strip_prefix("--") {
                let mut parts = stripped.splitn(2, '=');
                if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
                    map.insert(key.to_owned(), ConfigValue::Value(val.to_owned()));
                }
            }
        }

        Ok(ConfigValue::Section(map))
    }

    fn priority(&self) -> u8 {
        self.priority
    }
}
