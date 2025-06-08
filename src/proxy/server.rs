use std::net::SocketAddr;

use tokio::{io::AsyncReadExt, net::TcpListener};

use super::{error::HttpError, http::HttpRequest};

pub struct HttpServer {
    listener: TcpListener,
    addr: SocketAddr,
}

impl HttpServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, HttpError> {
        let listener = TcpListener::bind(&addr).await?;
        Ok(Self { listener, addr })
    }

    pub async fn run(self) -> Result<(), HttpError> {
        println!("HTTP server running at http://{}", self.addr);

        loop {
            match self.listener.accept().await {
                Ok((mut stream, client_addr)) => {
                    println!("New connection from client: {}", client_addr);

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(&mut stream).await {
                            eprintln!("Error handling connection: {}", e)
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e)
                }
            }
        }
    }
}

async fn handle_connection(stream: &mut tokio::net::TcpStream) -> Result<(), HttpError> {
    let mut buffer = [0u8; 4096];
    let bytes_read = stream.read(&mut buffer).await?;

    if bytes_read == 0 {
        return Ok(());
    }

    println!("Bytes read: {}", bytes_read);

    match HttpRequest::parse(&buffer[..bytes_read]) {
        Ok(request) => {
            println!("Parse request: {:#?}", request);
        }
        Err(e) => {
            eprintln!("Failed to parse HTTP request: {}", e);
        }
    };

    Ok(())
}
