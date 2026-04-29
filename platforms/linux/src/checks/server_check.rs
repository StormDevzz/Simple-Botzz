use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{Write, Read};

/// Результат проверки сервера
#[derive(Debug, Clone)]
pub struct ServerCheckResult {
    pub reachable: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

/// Проверяет доступность сервера
pub fn check_server(server: &str, port: u16) -> ServerCheckResult {
    let address = format!("{}:{}", server, port);
    
    let start = std::time::Instant::now();
    
    match TcpStream::connect_timeout(
        &address.to_socket_addrs().unwrap().next().unwrap(),
        Duration::from_secs(5)
    ) {
        Ok(_) => {
            let latency = start.elapsed().as_millis() as u64;
            ServerCheckResult {
                reachable: true,
                latency_ms: Some(latency),
                error: None,
            }
        }
        Err(e) => {
            ServerCheckResult {
                reachable: false,
                latency_ms: None,
                error: Some(e.to_string()),
            }
        }
    }
}

/// Проверяет, является ли сервер Minecraft сервером
pub fn check_minecraft_server(server: &str, port: u16) -> ServerCheckResult {
    let address = format!("{}:{}", server, port);
    
    match TcpStream::connect_timeout(
        &address.to_socket_addrs().unwrap().next().unwrap(),
        Duration::from_secs(5)
    ) {
        Ok(mut stream) => {
            // Отправляем handshake для Minecraft
            let handshake = vec![0x00, 0x00]; // Простой handshake
            if let Err(e) = stream.write_all(&handshake) {
                return ServerCheckResult {
                    reachable: false,
                    latency_ms: None,
                    error: Some(format!("Failed to send handshake: {}", e)),
                };
            }
            
            // Пытаемся прочитать ответ
            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let start = std::time::Instant::now();
                    let latency = start.elapsed().as_millis() as u64;
                    ServerCheckResult {
                        reachable: true,
                        latency_ms: Some(latency),
                        error: None,
                    }
                }
                Err(e) => {
                    ServerCheckResult {
                        reachable: false,
                        latency_ms: None,
                        error: Some(format!("Failed to read response: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            ServerCheckResult {
                reachable: false,
                latency_ms: None,
                error: Some(e.to_string()),
            }
        }
    }
}
