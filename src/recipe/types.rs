use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub schema_version: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub category: Vec<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub auth_types: Vec<String>,
    #[serde(default)]
    pub estimated_time: Option<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub outputs: Vec<RecipeOutput>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub gotchas: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub last_verified: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOutput {
    pub key: String,
    pub description: String,
    #[serde(default)]
    pub sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
    Info {
        id: String,
        message: String,
    },
    OpenUrl {
        id: String,
        url: String,
        message: String,
    },
    PromptConfirm {
        id: String,
        message: String,
    },
    PromptInput {
        id: String,
        message: String,
        output_key: String,
        #[serde(default)]
        validation: Option<String>,
        #[serde(default)]
        validation_error: Option<String>,
    },
    PromptChoice {
        id: String,
        message: String,
        choices: Vec<Choice>,
    },
    Validate {
        id: String,
        method: String,
        message: String,
        #[serde(default)]
        depends_on: Vec<String>,
        #[serde(default)]
        on_success: Option<String>,
        #[serde(default)]
        on_failure: Option<String>,
        /// Arbitrary config passed to the validator (e.g. url, header_name for http_get)
        #[serde(default)]
        config: HashMap<String, String>,
    },
    Output {
        id: String,
        message: String,
    },
    RunCommand {
        id: String,
        command: String,
        message: String,
    },
    Wait {
        id: String,
        message: String,
        #[serde(default)]
        resume_hint: Option<String>,
    },
    CopyToClipboard {
        id: String,
        value: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub label: String,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(default)]
    pub sets: Option<HashMap<String, String>>,
}

impl Step {
    pub fn id(&self) -> &str {
        match self {
            Step::Info { id, .. } => id,
            Step::OpenUrl { id, .. } => id,
            Step::PromptConfirm { id, .. } => id,
            Step::PromptInput { id, .. } => id,
            Step::PromptChoice { id, .. } => id,
            Step::Validate { id, .. } => id,
            Step::Output { id, .. } => id,
            Step::RunCommand { id, .. } => id,
            Step::Wait { id, .. } => id,
            Step::CopyToClipboard { id, .. } => id,
        }
    }
}

impl Recipe {
    pub fn find_step_index(&self, step_id: &str) -> Option<usize> {
        self.steps.iter().position(|s| s.id() == step_id)
    }
}
