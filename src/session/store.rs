use std::fs;
use std::path::PathBuf;

use crate::error::{GetapiError, Result};
use crate::session::types::Session;

fn sessions_dir() -> PathBuf {
    PathBuf::from(".getapi").join("sessions")
}

fn session_path(provider: &str) -> PathBuf {
    sessions_dir().join(format!("{}.json", provider))
}

pub fn save(session: &Session) -> Result<()> {
    let dir = sessions_dir();
    fs::create_dir_all(&dir)?;
    let path = session_path(&session.provider);
    let json = serde_json::to_string_pretty(session)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load(provider: &str) -> Result<Session> {
    let path = session_path(provider);
    if !path.exists() {
        return Err(GetapiError::SessionError(format!(
            "No session found for '{}'. Start one with: getapi {}",
            provider, provider
        )));
    }
    let contents = fs::read_to_string(path)?;
    let session: Session = serde_json::from_str(&contents)
        .map_err(|e| GetapiError::SessionError(format!("Corrupt session file: {}", e)))?;
    Ok(session)
}

pub fn delete(provider: &str) -> Result<()> {
    let path = session_path(provider);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn delete_all() -> Result<()> {
    let dir = sessions_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

pub fn list_all() -> Result<Vec<Session>> {
    let dir = sessions_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut sessions = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(session) = serde_json::from_str::<Session>(&contents) {
                    sessions.push(session);
                }
            }
        }
    }
    Ok(sessions)
}
