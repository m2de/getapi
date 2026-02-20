use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetapiError {
    #[error("Provider '{0}' not found. Run `getapi list` to see available providers.")]
    ProviderNotFound(String),

    #[error("Recipe file not found: {0}")]
    RecipeFileNotFound(String),

    #[error("Invalid recipe: {0}")]
    InvalidRecipe(String),

    #[error("Step '{0}' not found in recipe")]
    StepNotFound(String),

    #[error("Template variable '{0}' not set. This step depends on a previous choice or input.")]
    TemplateVarNotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Validator '{0}' not found. Available validators: {1}")]
    ValidatorNotFound(String, String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("User cancelled the operation.")]
    UserCancelled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to fetch remote recipes: {0}")]
    RemoteFetch(String),
}

pub type Result<T> = std::result::Result<T, GetapiError>;
