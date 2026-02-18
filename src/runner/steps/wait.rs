use crate::error::Result;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(message: &str, resume_hint: Option<&str>, ctx: &mut RunContext) -> Result<()> {
    ui::print_pause(message);

    if let Some(hint) = resume_hint {
        ui::print_info(hint);
    }

    if !ctx.non_interactive {
        ctx.paused = true;
    }

    Ok(())
}
