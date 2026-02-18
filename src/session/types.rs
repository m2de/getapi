use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    InProgress,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub provider: String,
    pub recipe_version: Option<String>,
    pub started_at: String,
    pub updated_at: String,
    pub current_step: Option<String>,
    pub status: SessionStatus,
    pub completed_steps: Vec<String>,
    pub skipped_steps: Vec<String>,
    pub choices_made: HashMap<String, String>,
    pub output_file: String,
    pub output_format: String,
    pub notes: Option<String>,
}

impl Session {
    pub fn new(provider: &str, output_file: &str, output_format: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            provider: provider.to_string(),
            recipe_version: None,
            started_at: now.clone(),
            updated_at: now,
            current_step: None,
            status: SessionStatus::InProgress,
            completed_steps: Vec::new(),
            skipped_steps: Vec::new(),
            choices_made: HashMap::new(),
            output_file: output_file.to_string(),
            output_format: output_format.to_string(),
            notes: None,
        }
    }
}
