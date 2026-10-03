use colored::*;
use anyhow::Result;

use tokio::{ io::AsyncReadExt, net::TcpStream };

use crate::http::check_http_service;

// print out data about a specific port
pub async fn scan_port(ip: String, port: u16, web_crawl: bool) -> Result<()> {
    let mut conn = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    // First we test for a http service since the port seems to be open
    if check_http_service(&ip, &port, web_crawl).await.is_ok() {
        return Ok(());
    }

    // since http failed we display the banner of the tcp service
    let mut buf = [0u8; 1200];
    match conn.read(&mut buf).await {
        Ok(n) => if n == 0 {
            println!(
                "[RustyScan] [ {} ] < {} > | possible opened door -> {}",
                "TCP".red().underline(),
                "server returned nothing, but port is opened".red().bold(),
                port
            );
            return Ok(());
        }
        Err(_) => {
            println!(
                "[RustyScan] [ {} ] < {} > | possible opened door -> {}",
                "TCP".red().underline(),
                "banner read err, but port is opened".red().bold(),
                port
            );
        }
    }

    let sanitized_banner_buf = strip_ansi_escapes::strip(buf);
    println!(
        "[RustyScan] [ {} ] | Twisted Doornob -> {} | banner -> [ {} ]",
        "TCP BANNER".red().underline(),
        port,
        String::from_utf8_lossy(&sanitized_banner_buf).replace("\r", "\\r").replace("\n", "\\n").bright_purple().bold()
    );

    Ok(())
}
