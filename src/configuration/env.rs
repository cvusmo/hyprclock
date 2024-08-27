// src/configuration/env.rs
// github.com/cvusmo/hyprclock

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvConfig {
    pub environment: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        EnvConfig {
            environment: String::from("development"), // Default value
        }
    }
}
