pub mod context;
pub mod steps;

use crate::error::{GetapiError, Result};
use crate::recipe::types::{Recipe, Step};
use crate::runner::context::RunContext;
use crate::ui;

pub fn run(recipe: &Recipe, ctx: &mut RunContext) -> Result<()> {
    ui::print_header(&recipe.display_name, &recipe.description);

    if let Some(ref time) = recipe.estimated_time {
        ui::print_meta("Estimated time", time);
    }

    if !recipe.prerequisites.is_empty() {
        ui::print_meta("Prerequisites", &recipe.prerequisites.join(", "));
    }

    let mut i = ctx.current_step_index;

    while i < recipe.steps.len() {
        let step = &recipe.steps[i];
        let step_id = step.id().to_string();

        // Skip completed steps (for resume)
        if ctx.is_completed(&step_id) {
            i += 1;
            continue;
        }

        ctx.current_step_index = i;
        let step_num = i + 1;
        ui::print_step_counter(step_num, ctx.total_steps);

        execute_step(step, ctx)?;

        // If paused (wait step), break out of the loop
        if ctx.paused {
            return Ok(());
        }

        ctx.mark_completed(&step_id);

        // Handle jump (from choice with `next`)
        if let Some(ref target) = ctx.jump_to.take() {
            match recipe.find_step_index(target) {
                Some(idx) => {
                    i = idx;
                    continue;
                }
                None => {
                    return Err(GetapiError::StepNotFound(target.clone()));
                }
            }
        }

        i += 1;
    }

    // If we completed all steps and have collected values, write output
    if !ctx.collected.is_empty() && !ctx.paused {
        println!();
        crate::output::write_output(&ctx.output_format, &ctx.output_file, &ctx.collected)?;
        ui::print_success(&format!("Credentials written to {}", ctx.output_file));
    }

    if !ctx.paused {
        if ctx.collected.is_empty() {
            println!();
        }
        ui::print_success(&format!("{} setup complete!", recipe.display_name));

        if !recipe.gotchas.is_empty() {
            println!();
            ui::print_section("Things to know");
            for gotcha in &recipe.gotchas {
                ui::print_bullet(gotcha);
            }
        }
    }

    Ok(())
}

fn execute_step(step: &Step, ctx: &mut RunContext) -> Result<()> {
    match step {
        Step::Info { message, .. } => steps::info::handle(message, ctx),
        Step::OpenUrl { url, message, .. } => steps::open_url::handle(url, message, ctx),
        Step::PromptConfirm { message, .. } => steps::prompt_confirm::handle(message, ctx),
        Step::PromptInput {
            message,
            output_key,
            validation,
            validation_error,
            ..
        } => steps::prompt_input::handle(
            message,
            output_key,
            validation.as_deref(),
            validation_error.as_deref(),
            ctx,
        ),
        Step::PromptChoice {
            id,
            message,
            choices,
        } => steps::prompt_choice::handle(id, message, choices, ctx),
        Step::Validate {
            method,
            message,
            on_success,
            on_failure,
            config,
            ..
        } => steps::validate::handle(
            method,
            message,
            on_success.as_deref(),
            on_failure.as_deref(),
            config,
            ctx,
        ),
        Step::Output { message, .. } => steps::output::handle(message, ctx),
        Step::RunCommand {
            command, message, ..
        } => steps::run_command::handle(command, message, ctx),
        Step::Wait {
            message,
            resume_hint,
            ..
        } => steps::wait::handle(message, resume_hint.as_deref(), ctx),
        Step::CopyToClipboard { value, message, .. } => {
            steps::copy_clipboard::handle(value, message, ctx)
        }
    }
}
