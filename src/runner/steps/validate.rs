use std::collections::HashMap;

use crate::error::Result;
use crate::runner::context::RunContext;
use crate::ui;
use crate::validators;

pub fn handle(
    method: &str,
    message: &str,
    on_success: Option<&str>,
    on_failure: Option<&str>,
    config: &HashMap<String, String>,
    ctx: &RunContext,
) -> Result<()> {
    if ctx.non_interactive {
        println!("  {} Validate credentials using: {}", console::style("→").cyan(), method);
        return Ok(());
    }

    ui::print_info(message);

    match validators::run(method, &ctx.collected, config) {
        Ok(()) => {
            let msg = on_success.unwrap_or("Credentials validated successfully.");
            ui::print_success(msg);
            Ok(())
        }
        Err(e) => {
            let msg = on_failure.unwrap_or("Validation failed.");
            ui::print_warning(&format!("{} ({})", msg, e));
            // Validation failure is a warning, not a fatal error
            Ok(())
        }
    }
}
