mod modules;

use clap::Parser;
use modules::args::Args;
use modules::scanner;
use modules::ui;
use std::net::ToSocketAddrs;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Parse Args
    let args = Args::parse();

    // 2. Banner
    ui::print_banner();

    // 3. Resolve IP
    let target_addr_str = format!("{}:80", args.target);
    let ip = match target_addr_str.to_socket_addrs() {
        Ok(mut iter) => {
            if let Some(addr) = iter.next() {
                addr.ip()
            } else {
                ui::print_error("No IP addresses found for target.");
                return Ok(());
            }
        }
        Err(_) => {
            ui::print_error("Could not resolve target hostname.");
            ui::print_info("Please check if the address is correct (e.g., google.com or 192.168.1.1).");
            return Ok(());
        }
    };

    // 4. Show Config
    ui::print_config(&args.target, args.start_port, args.end_port, args.timeout, args.concurrency);

    // 5. Validation
    if args.start_port > args.end_port {
        ui::print_error("Start port cannot be greater than end port.");
        return Ok(());
    }

    // 6. Run Scan & Measure Time
    let start_time = Instant::now();
    
    let results = scanner::run_scan(
        ip,
        args.start_port,
        args.end_port,
        args.timeout,
        args.concurrency
    ).await;

    let duration = start_time.elapsed();

    // 7. Show Results with stats
    ui::print_results(&results, duration);

    Ok(())
}
