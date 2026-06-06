use crate::modules::scanner::ScanResult;
use colored::*;
use std::net::IpAddr;
use std::time::Duration;

pub fn print_banner() {
    println!(
        "{}",
        r#"
__     __               __
\ \   / /__ _ __ _  __ / _|_ __ _____ ___
 \ \ / / _ \ '__| |/ /|  _| '__/ _ \ \ /
  \ V /  __/ |  |   < | | | | |  __/>  <
   \_/ \___|_|  |_|\_\|_| |_|  \___/_/\_\

"#.bright_cyan()
    );
    println!("{}", "Vortex Network Scanner".bright_cyan().bold());
    println!("{}", "High-performance async port scanner in Rust\n".bright_black());
}

pub fn print_config(
    target: &IpAddr,
    start_port: u16,
    end_port: u16,
    timeout: u64,
    concurrency: usize,
    top_ports: Option<u16>,
    current: usize,
    total: usize,
) {
    println!("{}\n", "⚙️  Scan Configuration".bold());
    if total > 1 {
        println!("  📍 Target {}: {} (target {}/{})", " ".repeat(6), target, current, total);
    } else {
        println!("  📍 Target:        {}", target);
    }
    if let Some(n) = top_ports {
        println!("  🚀 Mode:          Top {} ports", n);
    } else {
        println!("  📋 Port range:    {}-{}", start_port, end_port);
    }
    println!("  ⏱️  Timeout:       {}ms", timeout);
    println!("  ⚡ Concurrency:   {}", concurrency);
    println!();
}

pub fn print_info(msg: &str) {
    println!("  {} {}", "[i]".bright_blue(), msg);
}

pub fn print_ok(msg: &str) {
    println!("  {} {}", "[✓]".bright_green(), msg);
}

pub fn print_warn(msg: &str) {
    println!("  {} {}", "[!]".bright_yellow(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("  {} {}", "[✗]".bright_red(), msg);
}

pub fn print_results(results: &[ScanResult], duration: Duration) {
    if results.is_empty() {
        println!("  {} No open ports found.", "[!]".bright_yellow());
        println!();
        return;
    }

    println!("  {} Open ports found: {}", "[✓]".bright_green(), results.len());
    println!(
        "  {} Scan completed in {:.2}s",
        "[i]".bright_blue(),
        duration.as_secs_f64()
    );
    println!();

    println!(
        "  {:<8} {:<22} {:<18} {}",
        "PORT".bold(),
        "SERVICE".bold(),
        "STATE".bold(),
        "BANNER".bold()
    );
    println!("  {}", "─".repeat(80).bright_black());

    for result in results {
        let state = if result.open {
            "🟢 OPEN".bright_green()
        } else {
            "🔴 CLOSED".bright_red()
        };

        let service = if result.service.is_empty() {
            "unknown".bright_black().to_string()
        } else {
            result.service.bright_cyan().to_string()
        };

        let banner = if result.banner.is_empty() {
            String::new()
        } else {
            format!(" {}", result.banner.bright_white())
        };

        println!(
            "  {:<8} {:<22} {:<18}{}",
            format!("{}/tcp", result.port).bright_white(),
            service,
            state,
            banner,
        );
    }
    println!();
}
