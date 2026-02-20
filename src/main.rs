mod cli;
mod error;
mod manifest;
mod output;
mod recipe;
mod runner;
mod session;
mod ui;
mod validators;

use clap::Parser;

use crate::cli::args::{Cli, Command, OutputFormat};
use crate::error::{GetapiError, Result};
use crate::output::env as env_output;
use crate::recipe::registry::RecipeRegistry;
use crate::runner::context::RunContext;
use crate::session::types::{Session, SessionStatus};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        match e {
            GetapiError::UserCancelled => {
                std::process::exit(0);
            }
            _ => {
                eprintln!("\n  {} {}", console::style("Error:").red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let mut registry = RecipeRegistry::new();
    if let Some(ref dir) = cli.recipe_dir {
        registry = registry.with_extra_dir(dir);
    }

    match &cli.command {
        Some(Command::Update) => cmd_update(),
        Some(Command::List { search, category }) => {
            cmd_list(&registry, search.clone(), category.clone())
        }
        Some(Command::Status) => cmd_status(&registry),
        Some(Command::Resume { provider }) => cmd_resume(&registry, provider, &cli),
        Some(Command::Validate { provider }) => cmd_validate(&registry, provider),
        Some(Command::Reset { provider }) => cmd_reset(provider.clone()),
        None => {
            if let Some(ref recipe_path) = cli.recipe {
                let recipe = recipe::loader::load_from_file(recipe_path)?;
                run_provider_setup(&recipe, &cli)
            } else if cli.providers.is_empty() {
                cmd_manifest(&registry, &cli)
            } else {
                for provider_name in &cli.providers {
                    let recipe = registry
                        .find(provider_name)
                        .ok_or_else(|| GetapiError::ProviderNotFound(provider_name.clone()))?
                        .clone();
                    run_provider_setup(&recipe, &cli)?;
                }
                Ok(())
            }
        }
    }
}

fn run_provider_setup(recipe: &recipe::types::Recipe, cli: &Cli) -> Result<()> {
    let output_file = cli
        .output_file
        .clone()
        .unwrap_or_else(|| default_output_file(&cli.output));

    // Check if credentials already exist (UAC-12)
    if !cli.non_interactive {
        let existing = env_output::read_existing(&output_file);
        let all_present = recipe.outputs.iter().all(|o| existing.contains_key(&o.key));

        if all_present && !recipe.outputs.is_empty() {
            ui::print_header(&recipe.display_name, &recipe.description);
            println!();
            ui::print_success(
                "All credentials for this provider already exist in your output file.",
            );
            println!();
            let keys: Vec<&str> = recipe.outputs.iter().map(|o| o.key.as_str()).collect();
            ui::print_info(&format!("Found: {}", keys.join(", ")));
            println!();

            let choice = dialoguer::Select::new()
                .with_prompt("What would you like to do?")
                .items(&[
                    "Validate existing credentials",
                    "Re-run setup (will overwrite)",
                    "Exit",
                ])
                .default(0)
                .interact()
                .map_err(|_| GetapiError::UserCancelled)?;

            match choice {
                0 => {
                    for step in &recipe.steps {
                        if let recipe::types::Step::Validate {
                            method,
                            message,
                            on_success,
                            on_failure,
                            config,
                            ..
                        } = step
                        {
                            let ctx =
                                RunContext::new(cli.output.clone(), output_file.clone(), false, 0);
                            let ctx = RunContext {
                                collected: existing,
                                ..ctx
                            };
                            return runner::steps::validate::handle(
                                method,
                                message,
                                on_success.as_deref(),
                                on_failure.as_deref(),
                                config,
                                &ctx,
                            );
                        }
                    }
                    ui::print_info("No validation step found for this provider.");
                    return Ok(());
                }
                1 => { /* continue with setup */ }
                _ => return Ok(()),
            }
        }
    }

    let total_steps = recipe.steps.len();
    let mut ctx = RunContext::new(
        cli.output.clone(),
        output_file.clone(),
        cli.non_interactive,
        total_steps,
    );

    let mut session = Session::new(
        &recipe.id,
        &output_file,
        &format!("{:?}", cli.output).to_lowercase(),
    );
    session.recipe_version = recipe.version.clone();

    runner::run(recipe, &mut ctx)?;

    // Update session
    session.completed_steps = ctx.completed_steps.clone();
    session.choices_made = ctx.choices_made.clone();
    session.updated_at = chrono::Utc::now().to_rfc3339();

    if ctx.paused {
        session.status = SessionStatus::Paused;
        if let Some(step) = recipe.steps.get(ctx.current_step_index) {
            session.current_step = Some(step.id().to_string());
        }
    } else {
        session.status = SessionStatus::Completed;
        session.current_step = None;
    }

    session::store::save(&session)?;

    Ok(())
}

fn cmd_resume(registry: &RecipeRegistry, provider: &str, cli: &Cli) -> Result<()> {
    let session = session::store::load(provider)?;

    let recipe = registry
        .find(provider)
        .ok_or_else(|| GetapiError::ProviderNotFound(provider.to_string()))?
        .clone();

    let output_file = cli
        .output_file
        .clone()
        .unwrap_or(session.output_file.clone());

    let total_steps = recipe.steps.len();
    let mut ctx = RunContext::new(
        cli.output.clone(),
        output_file.clone(),
        cli.non_interactive,
        total_steps,
    );

    // Restore session state
    ctx.completed_steps = session.completed_steps.clone();
    ctx.choices_made = session.choices_made.clone();

    // Restore template vars from choices
    for (step_id, chosen_label) in &session.choices_made {
        if let Some(recipe::types::Step::PromptChoice { choices, .. }) =
            recipe.steps.iter().find(|s| s.id() == step_id)
        {
            if let Some(chosen) = choices.iter().find(|c| &c.label == chosen_label) {
                if let Some(ref sets) = chosen.sets {
                    for (k, v) in sets {
                        ctx.set_var(k.clone(), v.clone());
                    }
                }
            }
        }
    }

    // Check .env for already-collected values
    let existing = env_output::read_existing(&output_file);
    for output_def in &recipe.outputs {
        if let Some(value) = existing.get(&output_def.key) {
            ctx.set_collected(output_def.key.clone(), value.clone());
        }
    }

    // Find resume point
    if let Some(ref current) = session.current_step {
        if let Some(idx) = recipe.find_step_index(current) {
            ctx.current_step_index = idx;
        }
    }

    ui::print_info(&format!("Resuming {} setup...", recipe.display_name));
    println!();

    runner::run(&recipe, &mut ctx)?;

    // Update session
    let mut updated_session = session;
    updated_session.completed_steps = ctx.completed_steps;
    updated_session.choices_made = ctx.choices_made;
    updated_session.updated_at = chrono::Utc::now().to_rfc3339();

    if ctx.paused {
        updated_session.status = SessionStatus::Paused;
        if let Some(step) = recipe.steps.get(ctx.current_step_index) {
            updated_session.current_step = Some(step.id().to_string());
        }
    } else {
        updated_session.status = SessionStatus::Completed;
        updated_session.current_step = None;
    }

    session::store::save(&updated_session)?;

    Ok(())
}

fn cmd_validate(registry: &RecipeRegistry, provider: &str) -> Result<()> {
    let recipe = registry
        .find(provider)
        .ok_or_else(|| GetapiError::ProviderNotFound(provider.to_string()))?;

    let output_file = session::store::load(provider)
        .map(|s| s.output_file)
        .unwrap_or_else(|_| ".env".to_string());

    let existing = env_output::read_existing(&output_file);

    let mut missing = Vec::new();
    for output_def in &recipe.outputs {
        if !existing.contains_key(&output_def.key) {
            missing.push(output_def.key.as_str());
        }
    }

    if !missing.is_empty() {
        ui::print_warning(&format!(
            "Missing credentials in {}: {}",
            output_file,
            missing.join(", ")
        ));
        ui::print_info(&format!("Run `getapi {}` to set them up.", provider));
        return Ok(());
    }

    for step in &recipe.steps {
        if let recipe::types::Step::Validate {
            method,
            message,
            on_success,
            on_failure,
            config,
            ..
        } = step
        {
            let ctx = RunContext::new(OutputFormat::Env, output_file.clone(), false, 0);
            let ctx = RunContext {
                collected: existing,
                ..ctx
            };
            return runner::steps::validate::handle(
                method,
                message,
                on_success.as_deref(),
                on_failure.as_deref(),
                config,
                &ctx,
            );
        }
    }

    ui::print_info("No validation step defined for this provider.");
    Ok(())
}

fn cmd_list(
    registry: &RecipeRegistry,
    search: Option<String>,
    category: Option<String>,
) -> Result<()> {
    let recipes = if let Some(ref query) = search {
        registry.search(query)
    } else if let Some(ref cat) = category {
        registry.filter_by_category(cat)
    } else {
        registry.all().iter().collect()
    };

    if recipes.is_empty() {
        ui::print_info("No providers found.");
        return Ok(());
    }

    println!();
    println!("  {}", console::style("Available providers").bold());
    println!();

    for recipe in recipes {
        print!(
            "  {}  {}",
            console::style(&recipe.id).cyan().bold(),
            recipe.display_name
        );
        println!("  {}", console::style(&recipe.description).dim());
    }

    Ok(())
}

fn cmd_status(registry: &RecipeRegistry) -> Result<()> {
    let sessions = session::store::list_all()?;

    println!();
    println!("  {}", console::style("Provider status").bold());
    println!();

    if sessions.is_empty() {
        ui::print_info("No sessions found. Run `getapi <provider>` to get started.");
        return Ok(());
    }

    for session in &sessions {
        let recipe = registry.find(&session.provider);
        let total_steps = recipe.map(|r| r.steps.len()).unwrap_or(0);
        let completed = session.completed_steps.len();

        let status_str = match session.status {
            SessionStatus::InProgress => console::style("in progress").yellow().to_string(),
            SessionStatus::Paused => console::style("paused").yellow().to_string(),
            SessionStatus::Completed => console::style("done ✓").green().to_string(),
        };

        let display_name = recipe
            .map(|r| r.display_name.as_str())
            .unwrap_or(&session.provider);

        let (display_completed, display_total) = if session.status == SessionStatus::Completed {
            (completed, completed)
        } else {
            (completed, total_steps)
        };

        let width = 12;
        let filled = if display_total > 0 {
            (display_completed * width) / display_total
        } else {
            0
        };
        let empty = width - filled;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        println!(
            "  {:<20} {}  {}/{}  {}",
            display_name, bar, display_completed, display_total, status_str
        );
    }

    println!();
    Ok(())
}

fn cmd_manifest(registry: &RecipeRegistry, _cli: &Cli) -> Result<()> {
    match manifest::loader::load()? {
        Some(manifest) => {
            println!();
            println!("  {}", console::style("Project API setup").bold());
            println!();

            for mp in &manifest.providers {
                let recipe = registry.find(&mp.id);
                let display = recipe
                    .map(|r| r.display_name.clone())
                    .unwrap_or_else(|| mp.id.clone());

                let required_str = if mp.required { "" } else { " (optional)" };

                println!(
                    "  {} {}{}",
                    console::style("▸").cyan(),
                    display,
                    console::style(required_str).dim()
                );

                if let Some(ref reason) = mp.reason {
                    println!("    {}", console::style(reason).dim());
                }
            }

            println!();
            let required: Vec<_> = manifest
                .providers
                .iter()
                .filter(|p| p.required)
                .map(|p| p.id.clone())
                .collect();

            if required.is_empty() {
                ui::print_info("No required providers. All are optional.");
            } else {
                ui::print_info(&format!("Run: getapi {}", required.join(" ")));
            }

            Ok(())
        }
        None => {
            println!();
            ui::print_info("No providers specified and no manifest found.");
            ui::print_info("Usage: getapi <provider> [providers...]");
            ui::print_info("Run `getapi list` to see available providers.");
            ui::print_info("Run `getapi --help` for full usage information.");
            Ok(())
        }
    }
}

fn cmd_update() -> Result<()> {
    ui::print_info("Fetching latest recipes from the remote repository...");
    match recipe::remote::fetch_and_cache_all() {
        Ok(recipes) => {
            ui::print_success(&format!(
                "Updated {} recipes. Run `getapi list` to see available providers.",
                recipes.len()
            ));
            Ok(())
        }
        Err(e) => {
            eprintln!("\n  {} {}", console::style("Warning:").yellow().bold(), e);
            std::process::exit(1);
        }
    }
}

fn cmd_reset(provider: Option<String>) -> Result<()> {
    match provider {
        Some(p) => {
            session::store::delete(&p)?;
            ui::print_success(&format!("Session for '{}' has been reset.", p));
        }
        None => {
            session::store::delete_all()?;
            ui::print_success("All sessions have been reset.");
        }
    }
    Ok(())
}

fn default_output_file(format: &OutputFormat) -> String {
    match format {
        OutputFormat::Env => ".env".to_string(),
        OutputFormat::Json => "credentials.json".to_string(),
        OutputFormat::Stdout => String::new(),
    }
}
