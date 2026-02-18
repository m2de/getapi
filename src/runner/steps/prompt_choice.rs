use dialoguer::Select;

use crate::error::{GetapiError, Result};
use crate::recipe::template;
use crate::recipe::types::Choice;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(
    step_id: &str,
    message: &str,
    choices: &[Choice],
    ctx: &mut RunContext,
) -> Result<()> {
    let expanded = template::expand(message, &ctx.vars)?;

    if ctx.non_interactive {
        ui::print_info(&expanded);
        for choice in choices.iter() {
            println!("    {} {}", console::style("·").dim(), choice.label);
        }
        return Ok(());
    }

    let labels: Vec<&str> = choices.iter().map(|c| c.label.as_str()).collect();

    let selection = Select::new()
        .with_prompt(&expanded)
        .items(&labels)
        .default(0)
        .interact()
        .map_err(|_| GetapiError::UserCancelled)?;

    let chosen = &choices[selection];

    // Record the choice
    ctx.choices_made
        .insert(step_id.to_string(), chosen.label.clone());

    // Apply any variable sets
    if let Some(ref sets) = chosen.sets {
        for (key, value) in sets {
            ctx.set_var(key.clone(), value.clone());
        }
    }

    // Set jump target if the choice specifies one
    if let Some(ref next) = chosen.next {
        ctx.jump_to = Some(next.clone());
    }

    Ok(())
}
