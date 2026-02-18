# getapi

> Stop Googling "how to get Twitter API key". Just run `getapi twitter`.

**getapi** is a CLI that walks you through setting up API credentials for popular services. Instead of hunting through documentation, signup flows, and developer portals, getapi gives you step-by-step interactive guidance and writes the credentials to your project when you're done.

## Quick Start

```sh
# Install
cargo install getapi

# Set up Twitter API credentials
getapi twitter

# Set up Stripe (writes to .env by default)
getapi stripe

# See all available providers
getapi list
```

## Install

### Cargo (from source)

```sh
cargo install getapi
```

### npm (prebuilt binaries)

```sh
npx getapi-cli twitter
```

### Binary download

Prebuilt binaries for macOS, Linux, and Windows are available on the [GitHub Releases](https://github.com/m2de/getapi/releases) page.

## Providers

getapi ships with 20 built-in provider recipes:

| Provider | Category | Estimated Time |
|----------|----------|---------------|
| Anthropic / Claude API | AI, LLM | ~5 min |
| Auth0 | Auth, Identity | ~5 min |
| Clerk | Auth, Identity | ~5 min |
| Cloudflare API | CDN, Edge, DNS | ~5 min |
| Discord Bot | Messaging, Social | ~10 min |
| Firebase | Database, Backend, Auth | ~5 min |
| GitHub API | Developer Tools, Git | ~3 min |
| Google Maps API | Maps, Location | ~5 min |
| Notion API | Productivity, Notes | ~5 min |
| OpenAI API | AI, LLM | ~5 min |
| Resend | Email, Messaging | ~3 min |
| SendGrid Email API | Email, Messaging | ~10 min |
| Shopify Admin API | Ecommerce, Payments | ~10 min |
| Slack API | Messaging, Productivity | ~10 min |
| Spotify API | Music, Entertainment | ~5 min |
| Stripe API | Payments, Fintech | ~5 min |
| Supabase | Database, Backend, Auth | ~5 min |
| Twilio SMS & Voice API | Messaging, SMS, Voice | ~10 min |
| Twitter / X API | Social, Messaging | ~10 min |
| Vercel API | Deployment, Hosting | ~3 min |

## Usage

### Basic setup

```sh
# Interactive setup for a provider
getapi <provider>

# Example
getapi openai
```

### Commands

```sh
getapi <provider>       # Start guided setup for a provider
getapi list             # List all available providers
getapi resume           # Resume a paused setup session
getapi status           # Show active sessions
```

### Options

```sh
--output <format>       # Output format: env, json, yaml (default: env)
--output-file <path>    # Write credentials to a specific file
--non-interactive       # Print steps without prompts (for CI/docs)
--recipe <path>         # Use a custom recipe JSON file
```

### Manifest file

Create a `getapi.toml` manifest in your project to declare which providers your project needs:

```toml
[providers]
stripe = { output = ".env" }
openai = { output = ".env" }
supabase = { output = ".env" }
```

Then run `getapi` with no arguments to set up all missing providers.

## For AI Agents

getapi is designed to work well with AI coding agents:

- **`getapi <provider> --help`** shows the full setup flow without running it
- **`getapi <provider> --non-interactive`** prints all steps as plain text, so an agent can relay instructions to a user or parse the credential requirements
- All provider recipes are JSON files in `providers/` and can be read directly

## Adding a Provider

The most common contribution is adding a new provider recipe. See [CONTRIBUTING.md](CONTRIBUTING.md) for the recipe JSON structure, step types, and testing instructions.

## How It Works

1. **Recipe files** in `providers/` define the step-by-step flow for each service as JSON
2. **The runner** walks the user through each step: opening URLs, prompting for input, offering choices, and validating credentials
3. **Sessions** track progress so you can pause and resume long setup flows
4. **Output** writes collected credentials to `.env`, JSON, or YAML files

## Licence

[MIT](LICENSE)
