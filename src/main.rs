use std::env::args;

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

    let mut set = tokio::task::JoinSet::new();
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        return Err(anyhow::anyhow!(
            "[RustyPorts] Invalid Syntax! ./exe [IPV4] [start port] [end port]"
        ));
    }

    let ip: String = String::from(args[1].clone());
    let start_port: u16;
    let end_port: u16;

    match args[2].parse::<u16>() {
        Ok(port) => start_port = port,
        Err(_) => {
            return Err(anyhow::anyhow!("[RustyPorts] Invalid Syntax! ./exe [IPV4] Must be a port number! -> [start port] <- [end port]"))
        }
    }

    match args[3].parse::<u16>() {
        Ok(port) => end_port = port,
        Err(_) => {
            return Err(anyhow::anyhow!("[RustyPorts] Invalid Syntax! ./exe [IPV4] Must be a port number! [start port] -> [end port] <-"))
        }
    }

    println!("[RustyPorts] scanning {}-{}", start_port, end_port);

    for port in start_port..end_port {
        set.spawn(connect::scan_port(ip.clone(), port));
    }

    set.join_all().await;

    Ok(())
}
