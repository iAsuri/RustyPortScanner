use std::env::args;

use anyhow::bail;

mod http;
mod connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        r"                                                                           
     _____         _       _____         _   _____                         
    | __  |_ _ ___| |_ _ _|  _  |___ ___| |_|   __|___ ___ ___ ___ ___ ___ 
    |    -| | |_ -|  _| | |   __| . |  _|  _|__   |  _| .'|   |   | -_|  _|
    |__|__|___|___|_| |_  |__|  |___|_| |_| |_____|___|__,|_|_|_|_|___|_|  
                      |___|                                                
"
    );

    let args: Vec<String> = args().collect();

    if args.len() < 4 {
        bail!(
            "[RustyPorts] Invalid Syntax! ./exe [IPV4] [start port] [end port / or \"0\" for {} ports to be scanned] [webcrawl/true/false]",
            u16::MAX
        );
    }

    let ip: String = String::from(args[1].clone());

    let Ok(start_port) = args[2].parse::<u16>() else {
        bail!(
            "[RustyPorts] Invalid Syntax! ./exe [IPV4] Must be a port number! -> [start port] <- [end port]"
        );
    };

    let Ok(mut end_port) = args[3].parse::<u16>() else {
        bail!(
            "[RustyPorts] Invalid Syntax! ./exe [IPV4] Must be a port number! [start port] -> [end port] <-"
        );
    };

    if end_port == 0 {
        end_port = u16::MAX;
    }

    // not needed n im lazy to add a lib to parse cmdline args :P
    let webcrawl = args.len() > 4 && args[4].to_lowercase() == "true";

    println!("[RustyPorts] scanning {}-{}, Webcrawler: {}", start_port, end_port, webcrawl);

    let mut futures = vec![];

    for port in start_port..end_port {
        futures.push(connect::scan_port(ip.clone(), port, webcrawl));
    }

    futures::future::join_all(futures).await;

    Ok(())
}
