use dialoguer::Input;
use regex::Regex;

use crate::error::{GetapiError, Result};
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(
    message: &str,
    output_key: &str,
    validation: Option<&str>,
    validation_error: Option<&str>,
    ctx: &mut RunContext,
) -> Result<()> {
    let expanded = template::expand(message, &ctx.vars)?;

    if ctx.non_interactive {
        println!(
            "  {} {} {} ${}",
            console::style("→").cyan(),
            expanded,
            console::style("→").dim(),
            output_key
        );
        return Ok(());
    }

    let validation_regex = match validation {
        Some(pattern) => Some(
            Regex::new(pattern)
                .map_err(|e| GetapiError::InvalidRecipe(format!("Bad validation regex: {}", e)))?,
        ),
        None => None,
    };

    loop {
        let value: String = Input::new()
            .with_prompt(&expanded)
            .interact_text()
            .map_err(|_| GetapiError::UserCancelled)?;

        let value = value.trim().to_string();

        if value.is_empty() {
            ui::print_warning("Value cannot be empty. Please try again.");
            continue;
        }

        if let Some(ref re) = validation_regex {
            if !re.is_match(&value) {
                let msg = validation_error
                    .unwrap_or("Input doesn't match the expected format. Please try again.");
                ui::print_warning(msg);
                continue;
            }
        }

        ctx.set_collected(output_key.to_string(), value);
        break;
    }

    Ok(())
}
