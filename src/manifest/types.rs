use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub providers: Vec<ManifestProvider>,
    #[serde(default)]
    pub output: Option<ManifestOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestProvider {
    pub id: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default = "default_true")]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestOutput {
    #[serde(default = "default_env")]
    pub format: String,
    #[serde(default = "default_env_file")]
    pub file: String,
}

fn default_true() -> bool {
    true
}

fn default_env() -> String {
    "env".to_string()
}

fn default_env_file() -> String {
    ".env".to_string()
}
