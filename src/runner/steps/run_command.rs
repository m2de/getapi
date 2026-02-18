use std::process::Command;

use crate::error::Result;
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(command: &str, message: &str, ctx: &RunContext) -> Result<()> {
    let expanded_msg = template::expand(message, &ctx.vars)?;
    let expanded_cmd = template::expand(command, &ctx.vars)?;

    if ctx.non_interactive {
        println!("  {} {}", console::style("→").cyan(), expanded_msg);
        ui::print_command(&expanded_cmd);
        return Ok(());
    }

    ui::print_info(&expanded_msg);
    ui::print_command(&expanded_cmd);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&expanded_cmd)
        .status()?;

    if output.success() {
        ui::print_success("Command completed successfully.");
    } else {
        ui::print_warning(&format!(
            "Command exited with status {}. You may need to run it manually.",
            output.code().unwrap_or(-1)
        ));
    }

    Ok(())
}
