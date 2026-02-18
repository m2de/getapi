use std::collections::HashMap;

use crate::cli::args::OutputFormat;

pub struct RunContext {
    /// Template variables (from choices, etc.)
    pub vars: HashMap<String, String>,
    /// Collected credential values (output keys → values)
    pub collected: HashMap<String, String>,
    /// Steps that have been completed (by id)
    pub completed_steps: Vec<String>,
    /// Choices made (step_id → chosen label)
    pub choices_made: HashMap<String, String>,
    /// Output format
    pub output_format: OutputFormat,
    /// Output file path
    pub output_file: String,
    /// Whether running in non-interactive mode
    pub non_interactive: bool,
    /// Current step index (0-based)
    pub current_step_index: usize,
    /// Total number of steps
    pub total_steps: usize,
    /// Whether the session was paused (hit a wait step)
    pub paused: bool,
    /// The step id to jump to (from a choice with `next`)
    pub jump_to: Option<String>,
}

impl RunContext {
    pub fn new(
        output_format: OutputFormat,
        output_file: String,
        non_interactive: bool,
        total_steps: usize,
    ) -> Self {
        Self {
            vars: HashMap::new(),
            collected: HashMap::new(),
            completed_steps: Vec::new(),
            choices_made: HashMap::new(),
            output_format,
            output_file,
            non_interactive,
            current_step_index: 0,
            total_steps,
            paused: false,
            jump_to: None,
        }
    }

    pub fn set_var(&mut self, key: String, value: String) {
        self.vars.insert(key, value);
    }

    pub fn set_collected(&mut self, key: String, value: String) {
        self.collected.insert(key, value);
    }

    pub fn mark_completed(&mut self, step_id: &str) {
        if !self.completed_steps.contains(&step_id.to_string()) {
            self.completed_steps.push(step_id.to_string());
        }
    }

    pub fn is_completed(&self, step_id: &str) -> bool {
        self.completed_steps.contains(&step_id.to_string())
    }
}
