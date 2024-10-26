use tokio::{io::AsyncReadExt, net};

// print out data about a specific port
pub async fn scan_port(ip: String, port: u16) {
    match net::TcpStream::connect(format!("{}:{}", ip, port)).await {
        
        Ok(mut conn) => {
            if conn.readable().await.is_err() {
                return;
            }
            
            let mut buf = vec![0 as u8; 1200];
            
            match conn.read(&mut buf).await {
                Ok(read) => {
                    if read > 0 {
                        println!(
                            "[RustyPort] Twisted Doornob [{}] and got flashed with a banner [{:?}]",
                            port,
                            String::from_utf8_lossy(&buf).trim_end_matches('\0') // trim off null bytes
                        );
                    } else {
                        println!("[RustyPort] Possible Opened Door {}", port);
                    }
                }
                Err(_) => return,
            }
        }
        Err(_) => return,
    }
}
