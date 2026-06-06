use crate::modules::ports;
use std::net::{IpAddr, ToSocketAddrs};
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub port: u16,
    pub open: bool,
    pub service: &'static str,
    pub banner: String,
}

/// Quick TCP ping - try connecting to common ports to check if host is alive
pub async fn ping_host(ip: IpAddr, timeout_ms: u64) -> bool {
    let common_ports = [22, 80, 443, 3389, 8080];
    let duration = Duration::from_millis(timeout_ms);

    for port in common_ports {
        let addr = format!("{}:{}", ip, port);
        if timeout(duration, TcpStream::connect(&addr)).await.is_ok() {
            return true;
        }
    }
    false
}

/// Try to read a banner from an open TCP connection
async fn grab_banner(ip: IpAddr, port: u16, timeout_ms: u64) -> String {
    let addr = format!("{}:{}", ip, port);
    let duration = Duration::from_millis(timeout_ms);

    match timeout(duration, TcpStream::connect(&addr)).await {
        Ok(Ok(mut stream)) => {
            // Try to read up to 1024 bytes
            let mut buf = vec![0u8; 1024];
            match timeout(Duration::from_millis(2000), stream.read(&mut buf[..])).await {
                Ok(Ok(n)) if n > 0 => {
                    // Sanitize: only keep printable ASCII
                    buf.truncate(n);
                    buf.retain(|&b| b.is_ascii_graphic() || b == b' ' || b == b'\n' || b == b'\r' || b == b'\t');
                    let s = String::from_utf8_lossy(&buf).trim().to_string();
                    if s.len() > 80 {
                        format!("{}...", &s[..80])
                    } else {
                        s
                    }
                }
                _ => String::new(),
            }
        }
        _ => String::new(),
    }
}

/// Expand a CIDR range (e.g. "192.168.1.0/24") into individual IPs
pub fn expand_cidr(cidr: &str) -> Vec<IpAddr> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return vec![];
    }

    let prefix_len: u8 = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return vec![],
    };

    if prefix_len > 32 {
        return vec![];
    }

    let base: u32 = match parts[0].parse::<std::net::Ipv4Addr>() {
        Ok(ip) => u32::from(ip),
        Err(_) => return vec![],
    };

    let mask = if prefix_len == 0 {
        0
    } else {
        !0u32 << (32 - prefix_len)
    };
    let network = base & mask;
    let host_count = if prefix_len == 32 {
        1
    } else {
        2u32.pow((32 - prefix_len) as u32)
    };

    (0..host_count)
        .map(|i| IpAddr::V4(std::net::Ipv4Addr::from(network | i)))
        .collect()
}

/// Parse targets from --target (single) or --targets (file)
pub fn parse_targets(args: &crate::modules::args::Args) -> Vec<IpAddr> {
    let mut targets = Vec::new();

    // Single target (supports CIDR)
    if let Some(ref target) = args.target {
        if target.contains('/') {
            targets.extend(expand_cidr(target));
        } else {
            let addr_str = format!("{}:80", target);
            if let Ok(mut iter) = addr_str.to_socket_addrs() {
                if let Some(addr) = iter.next() {
                    targets.push(addr.ip());
                }
            }
        }
    }

    // Targets file
    if let Some(ref file_path) = args.targets {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if line.contains('/') {
                    targets.extend(expand_cidr(line));
                } else {
                    let addr_str = format!("{}:80", line);
                    if let Ok(mut iter) = addr_str.to_socket_addrs() {
                        if let Some(addr) = iter.next() {
                            targets.push(addr.ip());
                        }
                    }
                }
            }
        }
    }

    targets
}

/// Export scan results to JSON or CSV
pub fn export_results(results: &[ScanResult], path: &str) -> Result<(), std::io::Error> {
    if path.ends_with(".json") {
        let json = serde_json::to_string_pretty(&results.iter().map(|r| {
            serde_json::json!({
                "port": r.port,
                "open": r.open,
                "service": r.service,
                "banner": r.banner,
            })
        }).collect::<Vec<_>>())?;
        std::fs::write(path, json)?;
    } else if path.ends_with(".csv") {
        let mut csv = String::from("port,open,service,banner\n");
        for r in results {
            csv.push_str(&format!(
                "{},{},{},{}\n",
                r.port,
                r.open,
                r.service.replace(',', " "),
                r.banner.replace(',', " ").replace('\n', " "),
            ));
        }
        std::fs::write(path, csv)?;
    }
    Ok(())
}

/// Run the port scan (main scanning logic)
pub async fn run_scan(
    ip: IpAddr,
    start_port: u16,
    end_port: u16,
    timeout_ms: u64,
    concurrency: usize,
    verbose: bool,
    top_ports: Option<u16>,
) -> Vec<ScanResult> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut handles = Vec::new();
    let duration = Duration::from_millis(timeout_ms);

    // Determine port range
    let ports: Vec<u16> = if let Some(n) = top_ports {
        ports::top_ports(n)
    } else {
        (start_port..=end_port).collect()
    };

    for port in ports {
        let sem = Arc::clone(&semaphore);
        let ip = ip;

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await;
            let addr = format!("{}:{}", ip, port);

            let result = timeout(duration, TcpStream::connect(&addr)).await;
            match result {
                Ok(Ok(_)) => {
                    let service = ports::get_service_name(port);
                    let banner = if verbose {
                        grab_banner(ip, port, timeout_ms).await
                    } else {
                        String::new()
                    };
                    ScanResult {
                        port,
                        open: true,
                        service,
                        banner,
                    }
                }
                _ => ScanResult {
                    port,
                    open: false,
                    service: "",
                    banner: String::new(),
                },
            }
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            if result.open {
                results.push(result);
            }
        }
    }

    results.sort_by_key(|r| r.port);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_cidr_small() {
        let ips = expand_cidr("192.168.1.0/30");
        // 4 IPs: .0, .1, .2, .3
        assert_eq!(ips.len(), 4);
        assert_eq!(ips[0].to_string(), "192.168.1.0");
        assert_eq!(ips[3].to_string(), "192.168.1.3");
    }

    #[test]
    fn test_expand_cidr_single() {
        let ips = expand_cidr("10.0.0.1/32");
        assert_eq!(ips.len(), 1);
        assert_eq!(ips[0].to_string(), "10.0.0.1");
    }

    #[test]
    fn test_expand_cidr_invalid() {
        assert!(expand_cidr("not-cidr").is_empty());
        assert!(expand_cidr("1.2.3.4/33").is_empty());
        assert!(expand_cidr("bad/24").is_empty());
    }
}
