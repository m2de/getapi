# Contributing to getapi

The most impactful contribution you can make is **adding a new provider recipe**. Each recipe is a self-contained JSON file that guides users through setting up API credentials for a service.

## Recipe structure

Provider recipes live in `providers/<id>.json`. Here's the minimal structure:

```json
{
  "schema_version": "1",
  "id": "my-service",
  "display_name": "My Service API",
  "description": "Set up API access for My Service.",
  "category": ["category"],
  "website": "https://my-service.dev",
  "auth_types": ["api_key"],
  "estimated_time": "5 minutes",
  "prerequisites": [
    "A My Service account"
  ],
  "outputs": [
    {
      "key": "MY_SERVICE_API_KEY",
      "description": "API key for My Service",
      "sensitive": true
    }
  ],
  "steps": [
    ...
  ]
}
```

### Top-level fields

| Field | Required | Description |
|-------|----------|-------------|
| `schema_version` | Yes | Always `"1"` |
| `id` | Yes | Lowercase, hyphenated identifier (matches filename) |
| `display_name` | Yes | Human-readable name shown to users |
| `description` | Yes | One-line description of what the recipe sets up |
| `category` | Yes | Array of category tags (e.g. `["ai", "llm"]`) |
| `website` | Yes | URL to the service's developer portal |
| `auth_types` | Yes | Array: `"api_key"`, `"oauth2"`, `"bearer_token"` |
| `estimated_time` | Yes | Human-readable time estimate |
| `prerequisites` | Yes | Array of things the user needs before starting |
| `outputs` | Yes | Array of credential objects the recipe collects |
| `steps` | Yes | Array of step objects (see below) |

### Output objects

```json
{
  "key": "ENV_VAR_NAME",
  "description": "What this credential is",
  "sensitive": true
}
```

Set `sensitive: true` for secrets (API keys, tokens). These are masked in terminal output.

## Step types

Every step must have a unique `id` (string) and a `type`. Most steps also have a `message` field that supports `{{VARIABLE}}` template syntax.

### `info`

Display an informational message.

```json
{ "id": "intro", "type": "info", "message": "We'll set up your API key." }
```

### `open_url`

Open a URL in the user's browser.

```json
{
  "id": "open_console",
  "type": "open_url",
  "url": "https://console.example.com",
  "message": "Opening the developer console."
}
```

### `prompt_input`

Prompt the user for text input with optional validation.

```json
{
  "id": "collect_key",
  "type": "prompt_input",
  "message": "Paste your API key:",
  "output_key": "MY_SERVICE_API_KEY",
  "validation": "^sk-[a-zA-Z0-9]{32,}$",
  "validation_error": "That doesn't look like a valid API key."
}
```

| Field | Required | Description |
|-------|----------|-------------|
| `output_key` | Yes | Variable name to store the input |
| `validation` | No | Regex pattern the input must match |
| `validation_error` | No | Custom error shown on validation failure |

### `prompt_confirm`

Ask for yes/no confirmation. If the user says no, the session pauses for later resumption.

```json
{
  "id": "confirm_billing",
  "type": "prompt_confirm",
  "message": "Have you set up billing on your account?"
}
```

### `prompt_choice`

Present a multiple-choice menu. Choices can set variables or jump to different steps.

```json
{
  "id": "pick_mode",
  "type": "prompt_choice",
  "message": "Which keys do you need?",
  "choices": [
    { "label": "Test keys", "sets": { "mode": "test" } },
    { "label": "Live keys", "sets": { "mode": "live" }, "next": "live_warning" }
  ]
}
```

| Choice field | Required | Description |
|-------------|----------|-------------|
| `label` | Yes | Text shown for this option |
| `sets` | No | Variables to set when selected |
| `next` | No | Step ID to jump to after selection |

### `validate`

Validate credentials via HTTP request.

```json
{
  "id": "validate_key",
  "type": "validate",
  "method": "http_get",
  "depends_on": ["collect_key"],
  "message": "Checking your API key...",
  "on_success": "API key verified!",
  "on_failure": "Couldn't verify the key. Check it and try again.",
  "config": {
    "url": "https://api.example.com/v1/me",
    "header_name": "Authorization",
    "header_value": "Bearer {{MY_SERVICE_API_KEY}}"
  }
}
```

### `output`

Write collected credentials to a file.

```json
{ "id": "write_output", "type": "output", "message": "Saving your credentials." }
```

### `wait`

Pause the recipe so the user can complete an external action and resume later.

```json
{
  "id": "wait_for_approval",
  "type": "wait",
  "message": "Your application is being reviewed.",
  "resume_hint": "Run 'getapi resume' once approved."
}
```

### `open_url`

URLs and messages support `{{VARIABLE}}` templates that are substituted at runtime from previously collected values.

### `run_command`

Execute a shell command.

```json
{
  "id": "install_cli",
  "type": "run_command",
  "command": "npm install -g @example/cli",
  "message": "Installing the CLI tool."
}
```

### `copy_to_clipboard`

Copy a value to the system clipboard.

```json
{
  "id": "copy_token",
  "type": "copy_to_clipboard",
  "value": "{{MY_TOKEN}}",
  "message": "Token copied to clipboard."
}
```

## Testing your recipe

```sh
# Run your recipe from a file
cargo run -- --recipe providers/my-service.json

# Validate JSON syntax
python3 -m json.tool providers/my-service.json > /dev/null

# Run the full test suite
cargo test
```

## PR guidelines

1. **One provider per PR** -- keeps reviews focused
2. **Test the full flow** yourself before submitting
3. **Use existing recipes** as reference (see `providers/github.json` for a simple example, `providers/twitter.json` for a complex one)
4. **Keep step messages concise** -- users read these in a terminal
5. **Add validation** for `prompt_input` steps whenever the credential has a known format
6. **Include a `validate` step** if the service has a lightweight endpoint to test credentials against
