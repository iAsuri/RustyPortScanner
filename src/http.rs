use std::time::Duration;

use colored::*;
use anyhow::Result;
use reqwest::{ Client, StatusCode };

const FAMOUS_ENDPOINTS: [&str; 15] = [
    "/api/v1/users",
    "/api/v2/auth/login",
    "/swagger-ui.html",
    "/wp-json/wp/v2/",
    "/.well-known/security.txt",
    "/graphiql",
    "/graphql",
    "/metrics",
    "/actuator/health",
    "/.env",
    "/config.json",
    "/admin/dashboard",
    "/api/v1/health",
    "/v2/api-docs",
    "/api/v1/scrape",
];

pub async fn webcrawler(url: &str) -> Vec<String> {
    let mut uri = vec![];

    let mut futures = vec![];
    for path in FAMOUS_ENDPOINTS {
        let client = Client::builder().timeout(Duration::from_secs(10)).build().unwrap();
        futures.push(client.get(format!("{}{}", url, path)).send());
    }

    for get_result in futures::future::join_all(futures).await {
        let Ok(resp) = get_result else {
            continue;
        };

        let path = resp.url().path().to_string();
        if resp.status() == StatusCode::OK && !uri.contains(&path){
            uri.push(path);
        }
    }

    uri
}

// returns http service
pub async fn check_http_service(ip: &String, port: &u16, webcrawl: bool) -> Result<()> {
    let resp = reqwest::get(format!("http://{}:{}", ip, port)).await?;
    let hdrs = resp.headers();

    // Check if we can get the service name
    let service = if let Some(value) = hdrs.get("Server") {
        value.to_str()?
    } else {
        "service not found"
    };

    if !webcrawl {
        println!(
            "[RustyScan] [ {} ] | port -> {} | service: [ {} ]",
            "HTTP SERVICE".yellow().underline(),
            port,
            service.blue().underline()
        );
    }

    let endpoints = webcrawler(&format!("http://{}:{}", ip, port)).await;

    if endpoints.len() == 0 {
        println!(
            "[RustyScan] [ {} ] | port -> {} | service: [ {} ] | {}",
            "HTTP/CRAWL".yellow().underline(),
            port,
            service.blue().underline(),
            "NO ENDPOINTS FOUND".red().bold().underline()
        );
        return Ok(());
    }

    println!(
        "[RustyScan] [ {} ] | port -> {} | service: [ {} ]\r\n\t-- Endpoints Report Found: {}\r\n{}",
        "HTTP/CRAWL".yellow().underline(),
        port,
        service.blue().underline(),
        endpoints.len(),
        endpoints
            .iter()
            .map(|x| format!("\t   - {} [ {} ]", x.bright_purple().bold(), "200 OK".green().bold()))
            .collect::<Vec<_>>()
            .join("\r\n")
    );

    Ok(())
}
