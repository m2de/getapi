use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "getapi",
    about = "Guided, interactive walkthroughs for setting up developer API credentials.\n\n\
             getapi walks you through creating API keys for popular platforms like Twitter, \
             OpenAI, Stripe, and more. It opens the right pages, tells you what to click, \
             collects your credentials, validates they work, and writes them to .env.\n\n\
             Provider recipes are bundled. Run `getapi update` to fetch the latest recipes.",
    long_about = None,
    after_help = "\
EXAMPLES:\n  \
  getapi twitter              Set up Twitter API credentials\n  \
  getapi twitter openai       Set up multiple providers in sequence\n  \
  getapi                      Use manifest (.getapi/manifest.json) if present\n  \
  getapi list                 List all available providers\n  \
  getapi list --search email  Search for providers\n  \
  getapi resume twitter       Resume a paused session\n  \
  getapi status               Show setup progress for all providers\n  \
  getapi validate twitter     Re-validate existing credentials\n\n\
SESSIONS:\n  \
  Progress is saved automatically to .getapi/sessions/. Credentials are NEVER stored\n  \
  in sessions — only progress metadata. On resume, getapi checks your output file\n  \
  (.env by default) to detect already-collected credentials.\n\n\
OUTPUT:\n  \
  By default, credentials are written to .env in the current directory.\n  \
  Use --output to change format (env, json, stdout).\n  \
  Use --output-file to change the file path."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Provider names to set up (e.g. twitter openai stripe)
    #[arg(value_name = "PROVIDER")]
    pub providers: Vec<String>,

    /// Output format for collected credentials
    #[arg(long, short, default_value = "env", value_enum)]
    pub output: OutputFormat,

    /// Output file path (default: .env for env format, credentials.json for json)
    #[arg(long, value_name = "PATH")]
    pub output_file: Option<String>,

    /// Print setup steps as a checklist without interactive prompts
    #[arg(long)]
    pub non_interactive: bool,

    /// Use a custom recipe file instead of a bundled provider
    #[arg(long, value_name = "PATH")]
    pub recipe: Option<String>,

    /// Additional directory to search for provider recipes
    #[arg(long, value_name = "PATH")]
    pub recipe_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Resume a paused setup session
    Resume {
        /// Provider to resume
        provider: String,
    },

    /// Show setup progress for all providers
    Status,

    /// List available providers
    List {
        /// Search providers by name or description
        #[arg(long, short)]
        search: Option<String>,

        /// Filter by category
        #[arg(long, short)]
        category: Option<String>,
    },

    /// Re-run credential validation for a provider
    Validate {
        /// Provider to validate
        provider: String,
    },

    /// Clear session data and start fresh
    Reset {
        /// Provider to reset (omit to reset all)
        provider: Option<String>,
    },

    /// Update provider recipes from the remote repository
    Update,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Env,
    Json,
    Stdout,
}
