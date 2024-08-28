// src/configuration/env.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

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
