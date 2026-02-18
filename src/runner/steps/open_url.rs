use dialoguer::Confirm;

use crate::error::{GetapiError, Result};
use crate::recipe::template;
use crate::runner::context::RunContext;
use crate::ui;

pub fn handle(url: &str, message: &str, ctx: &RunContext) -> Result<()> {
    let expanded_msg = if ctx.non_interactive {
        template::expand_lenient(message, &ctx.vars)
    } else {
        template::expand(message, &ctx.vars)?
    };
    let expanded_url = if ctx.non_interactive {
        template::expand_lenient(url, &ctx.vars)
    } else {
        template::expand(url, &ctx.vars)?
    };

    ui::print_info(&expanded_msg);
    ui::print_url(&expanded_url);

    if !ctx.non_interactive {
        let open_browser = Confirm::new()
            .with_prompt("Open in browser?")
            .default(true)
            .interact()
            .map_err(|_| GetapiError::UserCancelled)?;

        if open_browser {
            if let Err(e) = open::that(&expanded_url) {
                ui::print_warning(&format!(
                    "Could not open browser automatically: {}. Open the URL above manually.",
                    e
                ));
            }
        }
    }
    Ok(())
}
