use crate::error::Result;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(message: &str, ctx: &RunContext) -> Result<()> {
    ui::print_info(message);

    match crate::output::write_output(&ctx.output_format, &ctx.output_file, &ctx.collected) {
        Ok(()) => {
            ui::print_success(&format!("Credentials written to {}", ctx.output_file));
        }
        Err(e) => {
            ui::print_warning(&format!("Failed to write output: {}", e));
        }
    }

    Ok(())
}
