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

    pub fn validate(&self) -> Result<(), String> {
        let valid_environments = vec!["development", "production"];
        if valid_environments.contains(&self.environment.as_str()) {
            Ok(())
        } else {
            Err(format!("Invalid environment: {}", self.environment))
        }
    }
}

