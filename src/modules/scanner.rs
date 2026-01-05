use crate::modules::ports;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::timeout as tokio_timeout;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub port: u16,
    pub service: &'static str,
}

pub async fn run_scan(
    target_ip: IpAddr,
    start_port: u16,
    end_port: u16,
    timeout_ms: u64,
    concurrency: usize,
) -> Vec<ScanResult> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let total_ports = end_port - start_port + 1;
    let pb = create_progress_bar(total_ports as u64);
    
    // Use a channel to stream results back instead of storing JoinHandles
    let (tx, mut rx) = mpsc::channel(concurrency + 100);

    for port in start_port..=end_port {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let timeout_duration = Duration::from_millis(timeout_ms);
        let pb_clone = pb.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let is_open = scan_single_port(target_ip, port, timeout_duration).await;
            drop(permit); // Release semaphore
            pb_clone.inc(1);
            
            if is_open {
                 let _ = tx_clone.send(ScanResult {
                    port,
                    service: ports::get_service_name(port),
                }).await;
            }
        });
    }

    // Drop the original sender so the channel closes when all tasks are done
    drop(tx);

    let mut results = vec![];
    while let Some(res) = rx.recv().await {
        results.push(res);
    }

    pb.finish_with_message("Scan completed!");
    results.sort_by_key(|k| k.port);
    results
}

async fn scan_single_port(ip: IpAddr, port: u16, timeout: Duration) -> bool {
    let addr = SocketAddr::new(ip, port);
    match tokio_timeout(timeout, TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}
