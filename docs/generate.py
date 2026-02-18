#!/usr/bin/env python3
"""Static site generator for getapi provider documentation."""

import json
import os
import re
import shutil
from datetime import datetime
from pathlib import Path

from markupsafe import Markup
from jinja2 import Environment, FileSystemLoader

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
ROOT = Path(__file__).resolve().parent.parent
PROVIDERS_DIR = ROOT / "providers"
TEMPLATES_DIR = Path(__file__).resolve().parent / "templates"
ASSETS_DIR = Path(__file__).resolve().parent / "assets"
SITE_DIR = ROOT / "site"

BASE_URL = os.environ.get("BASE_URL", "/getapi")
SITE_URL = os.environ.get("SITE_URL", "https://m2de.github.io/getapi")

# ---------------------------------------------------------------------------
# Step type display labels
# ---------------------------------------------------------------------------
STEP_TYPE_LABELS = {
    "info": "Info",
    "open_url": "Open URL",
    "prompt_input": "Input",
    "prompt_choice": "Choice",
    "prompt_confirm": "Confirm",
    "validate": "Validate",
    "wait": "Wait",
    "run_command": "Run Command",
    "output": "Output",
    "copy_to_clipboard": "Copy",
}


# ---------------------------------------------------------------------------
# Helper functions
# ---------------------------------------------------------------------------
def process_template_vars(text: str) -> Markup:
    """Replace {{var_name}} with styled <code> spans.

    This must run *before* Jinja2 rendering so that Jinja2 doesn't try to
    interpret recipe template variables as its own expressions.
    Returns a Markup object so Jinja2 autoescaping won't double-escape it.
    """
    if text is None:
        return Markup("")
    # First escape the text itself, then inject our safe HTML
    from markupsafe import escape
    escaped = escape(text)
    result = re.sub(
        r"\{\{(\w+)\}\}",
        r'<code class="template-var">\1</code>',
        str(escaped),
    )
    return Markup(result)


def humanize_regex(pattern: str | None) -> Markup | None:
    """Extract a human-friendly hint from a validation regex."""
    if not pattern:
        return None

    hints = []

    # Starts with literal prefix
    m = re.match(r"^\^([A-Za-z0-9_-]+)", pattern)
    if m:
        prefix = m.group(1)
        hints.append(f'Starts with <code>{prefix}</code>')

    # Character length from quantifier
    m = re.search(r"\{(\d+)\}$", pattern.rstrip("$"))
    if m:
        hints.append(f"{m.group(1)} characters")
    else:
        m = re.search(r"\{(\d+),(\d*)\}", pattern)
        if m:
            lo = m.group(1)
            hi = m.group(2)
            if hi:
                hints.append(f"{lo}\u2013{hi}+ characters")
            else:
                hints.append(f"At least {lo} characters")

    return Markup(". ".join(hints)) if hints else None


def parse_estimated_time(text: str | None) -> str:
    """Convert e.g. '5 minutes' to ISO 8601 duration 'PT5M'."""
    if not text:
        return "PT5M"
    m = re.search(r"(\d+)\s*min", text)
    if m:
        return f"PT{m.group(1)}M"
    return "PT5M"


def load_providers() -> list[dict]:
    """Load all provider JSON files, sorted by display_name."""
    providers = []
    for path in sorted(PROVIDERS_DIR.glob("*.json")):
        with open(path) as f:
            data = json.load(f)
        providers.append(data)
    providers.sort(key=lambda p: p.get("display_name", "").lower())
    return providers


def collect_categories(providers: list[dict]) -> list[str]:
    """Collect unique categories across all providers, sorted."""
    cats = set()
    for p in providers:
        for c in p.get("category", []):
            cats.add(c)
    return sorted(cats)


def prepare_steps(provider: dict) -> list[dict]:
    """Process step data for template rendering.

    - Adds message_html with template vars converted
    - Adds validation_hint from regex
    """
    steps = []
    for step in provider.get("steps", []):
        s = dict(step)  # shallow copy
        s["message_html"] = process_template_vars(s.get("message", ""))
        if s.get("validation"):
            s["validation_hint"] = humanize_regex(s["validation"])
        else:
            s["validation_hint"] = None
        steps.append(s)
    return steps


def generate_sitemap(provider_ids: list[str]) -> str:
    """Generate sitemap.xml content."""
    today = datetime.now().strftime("%Y-%m-%d")
    urls = [f"  <url><loc>{SITE_URL}/</loc><lastmod>{today}</lastmod><priority>1.0</priority></url>"]
    for pid in sorted(provider_ids):
        urls.append(
            f"  <url><loc>{SITE_URL}/{pid}/</loc><lastmod>{today}</lastmod><priority>0.8</priority></url>"
        )
    return (
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'
        + "\n".join(urls)
        + "\n</urlset>\n"
    )


def generate_robots() -> str:
    """Generate robots.txt content."""
    return f"User-agent: *\nAllow: /\n\nSitemap: {SITE_URL}/sitemap.xml\n"


# ---------------------------------------------------------------------------
# Main build
# ---------------------------------------------------------------------------
def build():
    # Clean output
    if SITE_DIR.exists():
        shutil.rmtree(SITE_DIR)
    SITE_DIR.mkdir(parents=True)

    # Set up Jinja2
    env = Environment(
        loader=FileSystemLoader(str(TEMPLATES_DIR)),
        autoescape=True,
        trim_blocks=True,
        lstrip_blocks=True,
    )

    # Load data
    providers = load_providers()
    categories = collect_categories(providers)
    provider_ids = [p["id"] for p in providers]

    files_written = []

    # --- Index page ---
    tpl_index = env.get_template("index.html")
    html = tpl_index.render(
        base_url=BASE_URL,
        providers=providers,
        categories=categories,
    )
    index_path = SITE_DIR / "index.html"
    index_path.write_text(html)
    files_written.append("index.html")

    # --- Provider pages ---
    tpl_provider = env.get_template("provider.html")
    for provider in providers:
        prepared_steps = prepare_steps(provider)
        provider_copy = dict(provider)
        provider_copy["steps"] = prepared_steps

        html = tpl_provider.render(
            base_url=BASE_URL,
            provider=provider_copy,
            step_type_labels=STEP_TYPE_LABELS,
            estimated_time_iso=parse_estimated_time(provider.get("estimated_time")),
        )
        out_dir = SITE_DIR / provider["id"]
        out_dir.mkdir(parents=True, exist_ok=True)
        (out_dir / "index.html").write_text(html)
        files_written.append(f"{provider['id']}/index.html")

    # --- 404 page ---
    tpl_404 = env.get_template("404.html")
    html = tpl_404.render(base_url=BASE_URL)
    (SITE_DIR / "404.html").write_text(html)
    files_written.append("404.html")

    # --- Copy assets ---
    assets_dest = SITE_DIR / "assets"
    shutil.copytree(str(ASSETS_DIR), str(assets_dest))
    for f in assets_dest.rglob("*"):
        if f.is_file():
            files_written.append(f"assets/{f.relative_to(assets_dest)}")

    # --- Sitemap ---
    sitemap = generate_sitemap(provider_ids)
    (SITE_DIR / "sitemap.xml").write_text(sitemap)
    files_written.append("sitemap.xml")

    # --- robots.txt ---
    robots = generate_robots()
    (SITE_DIR / "robots.txt").write_text(robots)
    files_written.append("robots.txt")

    # --- Summary ---
    print(f"Built {len(files_written)} files in site/:")
    for f in sorted(files_written):
        print(f"  {f}")
    print(f"\n{len(providers)} provider pages generated.")


if __name__ == "__main__":
    build()
