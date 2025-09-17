//use axum::http::StatusCode;
use std::net::SocketAddr;

#[derive(Debug)]
struct Config {
    localhost: String,
    port: i16,
}

impl Config {
    fn new() -> Self {
        Config {
            localhost: "127.0.0.1".to_string(),
            port: 3000,
        }
    }

    // Validar las direccion ip y puerto
    fn socket_addr(&self) -> Result<SocketAddr, String> {
        let socket_addr: SocketAddr = match self.localhost.parse() {
            Ok(ip) => SocketAddr::new(ip, self.port as u16),
            Err(_) => return Err(format!("Invalid localhost address: {}", self.localhost)),
        };
        Ok(socket_addr)
    }
}

async fn config(app: axum::Router) -> Result<(), String> {
    let config = Config::new();

    //let addr = format!("{}:{}", config.localhost, config.port);
    //se cambio por el codigo de abajo
    let addr = match config.socket_addr() {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };

    // let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // se cambio por el codigo de mas abajo
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(error) => match error.kind() {
            std::io::ErrorKind::AddrInUse => {
                return Err(format!("Address already in use: {}", error));
            }
            std::io::ErrorKind::PermissionDenied => {
                return Err(format!("Permission denied to bind to address: {}", error));
            }
            std::io::ErrorKind::AddrNotAvailable => {
                return Err(format!("Address not available: {}", addr));
            }
            _ => {
                return Err(format!("Failed to bind to address: {}", error));
            }
        },
    };

    // informacion de tracing
    tracing::info!(
        "🚀 Server running http://{}",
        listener.local_addr().unwrap()
    );

    //axum::serve(listener, app).await.unwrap();
    // se cambio por el codigo mas abajo
    match axum::serve(listener, app).await {
        Ok(server) => server,
        Err(error) => return Err(format!("Server error: {}", error)),
    };

    Ok(())
}

pub async fn server_config(app: axum::Router) -> Result<(), String> {
    match config(app).await {
        Ok(()) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

// test unitarios

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localhost_is_str_test() {
        let config = Config::new();
        assert!(config.localhost.is_ascii(), "String");
        assert!(!config.localhost.is_empty(), "Not empty");
    }

    #[test]
    fn port_is_i16_test() {
        let config = Config::new();
        assert!(config.port >= i16::MIN && config.port <= i16::MAX);
    }

    #[test]
    fn socket_addr_test_valid() {
        let config = Config::new();
        let addr = config.socket_addr();
        assert!(addr.is_ok(), "Expected valid socket address...!");
    }

    #[test]
    fn socket_addr_test_invalid() {
        let config = Config {
            localhost: "invalid_ip".to_string(),
            port: 3000,
        };

        let addr = config.socket_addr();
        assert!(addr.is_err(), "Expected error for invalid IP");
        assert_eq!(addr.unwrap_err(), "Invalid localhost address: invalid_ip");
    }

    #[tokio::test]
    async fn socket_addr_test_invalid_bind() {
        let config = Config {
            localhost: "255.256.257.258".to_string(),
            port: 3000,
        };

        let result = config.socket_addr();
        assert!(result.is_err(), "Expected invalid address error");
    }

    // verificar no usar la ip dos veces
    #[tokio::test]
    async fn socket_addr_test_in_use_error() {
        let addr = "127.0.0.1:0";
        let listener1 = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Fail bind 1");

        let addr_in_use = listener1.local_addr().unwrap();

        match tokio::net::TcpListener::bind(&addr_in_use).await {
            Ok(_) => panic!("Expected AddrInUse error, successful bind"),
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::AddrInUse);
            }
        }
        drop(listener1);
    }

    #[test]
    fn config_test() {
        let config = Config::new();
        assert_eq!(config.localhost, "127.0.0.1");
        assert_eq!(config.port, 3000);
    }
}
