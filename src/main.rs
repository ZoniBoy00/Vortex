mod modules;

use clap::Parser;
use modules::args::Args;
use modules::scanner;
use modules::ui;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Validate
    if args.target.is_none() && args.targets.is_none() {
        ui::print_error("Specify a target with --target or --targets");
        ui::print_info("Example: vortex --target google.com");
        return Ok(());
    }

    if args.start_port > args.end_port {
        ui::print_error("Start port cannot be greater than end port.");
        return Ok(());
    }

    ui::print_banner();

    // Parse targets (supports CIDR, single, or file)
    let targets = scanner::parse_targets(&args);

    if targets.is_empty() {
        ui::print_error("Could not resolve any targets.");
        return Ok(());
    }

    // Ping check
    if args.ping {
        for ip in &targets {
            ui::print_info(&format!("Pinging {}...", ip));
            if scanner::ping_host(*ip, args.timeout).await {
                ui::print_ok(&format!("{} is alive!", ip));
            } else {
                ui::print_warn(&format!("{} is not responding to ping.", ip));
            }
        }
    }

    let total_targets = targets.len();
    let start_time = Instant::now();

    for (idx, ip) in targets.iter().enumerate() {
        ui::print_config(ip, args.start_port, args.end_port, args.timeout, args.concurrency, args.top_ports, idx + 1, total_targets);

        let results = scanner::run_scan(
            *ip,
            args.start_port,
            args.end_port,
            args.timeout,
            args.concurrency,
            args.verbose,
            args.top_ports,
        ).await;

        let duration = start_time.elapsed();
        ui::print_results(&results, duration);

        // Export if requested
        if let Some(ref output_path) = args.output {
            let out_path = if total_targets > 1 {
                // Append target name for multi-target scans
                let base = output_path.trim_end_matches(".json").trim_end_matches(".csv");
                let ext = if output_path.ends_with(".json") { ".json" } else { ".csv" };
                format!("{}-{}{}", base, ip, ext)
            } else {
                output_path.clone()
            };
            match scanner::export_results(&results, &out_path) {
                Ok(_) => ui::print_ok(&format!("Results exported to {}", out_path)),
                Err(e) => ui::print_error(&format!("Export failed: {}", e)),
            }
        }
    }

    Ok(())
}
