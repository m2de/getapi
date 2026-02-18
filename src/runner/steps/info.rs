use crate::error::Result;
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(message: &str, ctx: &RunContext) -> Result<()> {
    let expanded = if ctx.non_interactive {
        template::expand_lenient(message, &ctx.vars)
    } else {
        template::expand(message, &ctx.vars)?
    };
    ui::print_info(&expanded);
    Ok(())
}
