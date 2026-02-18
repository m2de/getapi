use dialoguer::Confirm;

use crate::error::{GetapiError, Result};
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(message: &str, ctx: &RunContext) -> Result<()> {
    let expanded = template::expand(message, &ctx.vars)?;

    if ctx.non_interactive {
        println!("  {} {}", console::style("→").cyan(), expanded);
        return Ok(());
    }

    let confirmed = Confirm::new()
        .with_prompt(&expanded)
        .default(true)
        .interact()
        .map_err(|_| GetapiError::UserCancelled)?;

    if !confirmed {
        ui::print_warning("Take your time — you can resume later with `getapi resume`.");
        return Err(GetapiError::UserCancelled);
    }

    Ok(())
}
