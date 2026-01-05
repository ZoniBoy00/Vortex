use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    /// Target IP address or domain to scan
    #[arg(short, long)]
    pub target: String,

    /// Start port (default: 1)
    #[arg(short, long, default_value_t = 1)]
    pub start_port: u16,

    /// End port (default: 1000)
    #[arg(short, long, default_value_t = 1000)]
    pub end_port: u16,

    /// Timeout in milliseconds (default: 1000ms)
    #[arg(long, default_value_t = 1000)]
    pub timeout: u64,

    /// Number of concurrent threads/tasks (default: 1000)
    #[arg(short, long, default_value_t = 1000)]
    pub concurrency: usize,
}
