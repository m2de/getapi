use console::style;

pub fn print_header(name: &str, description: &str) {
    println!();
    println!("{}  {}", style("▸").cyan().bold(), style(name).bold());
    println!("  {}", style(description).dim());
}

pub fn print_meta(label: &str, value: &str) {
    println!("  {} {}", style(format!("{}:", label)).dim(), value);
}

pub fn print_step_counter(current: usize, total: usize) {
    println!();
    println!("  {}", style(format!("[{}/{}]", current, total)).dim());
}

pub fn print_info(message: &str) {
    println!("  {}", message);
}

pub fn print_url(url: &str) {
    println!("  {} {}", style("→").cyan(), style(url).underlined());
}

pub fn print_command(cmd: &str) {
    println!("  {} {}", style("$").dim(), style(cmd).bold());
}

pub fn print_success(message: &str) {
    println!("  {} {}", style("✓").green().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("  {} {}", style("!").yellow().bold(), message);
}

pub fn print_section(title: &str) {
    println!("  {}", style(title).bold().underlined());
}

pub fn print_bullet(message: &str) {
    println!("  {} {}", style("•").dim(), message);
}

pub fn print_pause(message: &str) {
    println!("  {} {}", style("⏸").cyan().bold(), message);
}
