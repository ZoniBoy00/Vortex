use crate::modules::scanner::ScanResult;
use colored::*;

pub fn print_banner() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen

    println!("{}", r#"
  _    __            __
 | |  / /___  ____  / /____  _  __
 | | / / __ \/ ___\/ __/ _ \| |/_/
 | |/ / /_/ / /   / /_/  __/>  <
 |___/\____/_/    \__/\___/_/|_|
                                     
    "#.bright_magenta().bold());
    println!("{}", "  VORTEX NETWORK SCANNER v0.1".bright_cyan());
    println!("{}", "  High-Performance Security Tool".blue().bold());
    println!();
}

pub fn print_config(target: &str, start: u16, end: u16, timeout: u64, threads: usize) {
    let border_color = |s: &str| s.bright_black();
    let label_color = |s: &str| s.bright_white();
    let val_color = |s: &str| s.cyan().bold();

    println!("  {}", border_color("┌── SCAN TARGET ───────────────────────────────┐"));
    println!("  {} {}: {:<31} {}", border_color("│"), label_color("Target     "), val_color(target), border_color("│"));
    println!("  {} {}: {:<31} {}", border_color("│"), label_color("Range      "), format!("{}-{}", start, end).yellow(), border_color("│"));
    println!("  {} {}: {:<31} {}", border_color("│"), label_color("Strategy   "), format!("{} threads, {}ms", threads, timeout).magenta(), border_color("│"));
    println!("  {}", border_color("└──────────────────────────────────────────────┘"));
    println!();
}

pub fn print_results(results: &[ScanResult], duration: std::time::Duration) {
    println!();

    if results.is_empty() {
        println!("  {}", "No open ports found.".red());
        return;
    }

    // Table Config
    let col_port_width = 10;
    let col_status_width = 10;
    let col_service_width = 25;
    
    // Draw Header
    let top_border = format!("  ┌{}┬{}┬{}┐", "─".repeat(col_port_width), "─".repeat(col_status_width), "─".repeat(col_service_width));
    let header = format!("  │{:<w1$}│{:<w2$}│{:<w3$}│", 
        " PORT".bold(), " STATUS".bold(), " SERVICE".bold(), 
        w1=col_port_width, w2=col_status_width, w3=col_service_width);
    let mid_border = format!("  ├{}┼{}┼{}┤", "─".repeat(col_port_width), "─".repeat(col_status_width), "─".repeat(col_service_width));
    let bot_border = format!("  └{}┴{}┴{}┘", "─".repeat(col_port_width), "─".repeat(col_status_width), "─".repeat(col_service_width));

    println!("{}", top_border.bright_black());
    println!("{}", header);
    println!("{}", mid_border.bright_black());

    // Draw Rows
    for r in results {
        let port_str = format!(" {}", r.port);
        let status_str = " OPEN";
        // Truncate service if too long
        let mut service_val = r.service.to_string();
        if service_val.len() > col_service_width - 2 {
            service_val.truncate(col_service_width - 5);
            service_val.push_str("...");
        }
        let service_str = format!(" {}", service_val);
        
        let row = format!("  │{:<w1$}│{:<w2$}│{:<w3$}│", 
            port_str.bold(), 
            status_str.green().bold(), 
            service_str, 
            w1=col_port_width, w2=col_status_width, w3=col_service_width
        );
        println!("{}", row);
    }
    
    println!("{}", bot_border.bright_black());

    // Summary
    let duration_secs = duration.as_secs_f64();
    println!();
    println!("  {}", "Scan Complete.".bright_green().bold());
    println!("  • {} open ports found", results.len().to_string().white().bold());
    println!("  • Time elapsed: {:.2}s", duration_secs);
    println!();
}

pub fn print_error(msg: &str) {
    println!("  {} {}", "x".red().bold(), msg.red());
}

pub fn print_info(msg: &str) {
    println!("  {} {}", "i".blue().bold(), msg.white());
}
