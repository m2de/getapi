use crate::error::Result;
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(value: &str, message: &str, ctx: &RunContext) -> Result<()> {
    let expanded_msg = template::expand(message, &ctx.vars)?;
    let expanded_val = template::expand(value, &ctx.vars)?;

    ui::print_info(&expanded_msg);

    if ctx.non_interactive {
        ui::print_info(&format!("  Value: {}", expanded_val));
        return Ok(());
    }

    match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(&expanded_val)) {
        Ok(()) => {
            ui::print_success("Copied to clipboard.");
        }
        Err(_) => {
            ui::print_warning("Could not copy to clipboard. Here's the value:");
            ui::print_info(&format!("  {}", expanded_val));
        }
    }

    Ok(())
}
