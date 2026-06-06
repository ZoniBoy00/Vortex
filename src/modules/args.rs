use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    /// Target IP address, domain, or CIDR range to scan
    /// Use --targets to scan multiple targets from a file
    #[arg(short, long)]
    pub target: Option<String>,

    /// Path to a file containing targets (one per line)
    #[arg(long)]
    pub targets: Option<String>,

    /// Start port (default: 1)
    #[arg(short = 's', long, default_value_t = 1)]
    pub start_port: u16,

    /// End port (default: 1000)
    #[arg(short = 'e', long, default_value_t = 1000)]
    pub end_port: u16,

    /// Scan only the top N most common ports (overrides --start-port/--end-port)
    #[arg(long)]
    pub top_ports: Option<u16>,

    /// Timeout in milliseconds for each port connection (default: 1000ms)
    #[arg(long, default_value_t = 1000)]
    pub timeout: u64,

    /// Number of concurrent tasks (default: 1000)
    #[arg(short, long, default_value_t = 1000)]
    pub concurrency: usize,

    /// Enable verbose output — shows banner info for open ports
    #[arg(short, long)]
    pub verbose: bool,

    /// Ping the target before scanning to check if it's alive (ICMP or TCP)
    #[arg(long)]
    pub ping: bool,

    /// Export results to a file (supports .json or .csv extension)
    #[arg(short = 'o', long)]
    pub output: Option<String>,
}
