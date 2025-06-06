use clap::Parser;
use serde::{Serialize};
use serde_json::to_string_pretty;
use std::{ time::Duration};
use tokio::net::TcpStream;

#[derive(Serialize, Debug)]
struct PortStatus {
    port: u16,
    is_open: bool,
}


#[derive(Debug)]
enum PortParseError {
    InvalidNumber,
    InvalidRange,
}

#[derive(Parser, Debug)]
struct Cli {
    host: String,

    ports: String,

    #[arg(short, long, default_value_t = 1000)]
    timeout: u64,

    #[arg(long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let ports_result = parse_ports(&args.ports);
    if let Err(e) = &ports_result {
        eprintln!("{:?}", e);
        return;
    }

    let ports = ports_result.unwrap();

    if !args.json{
        println!("\nScanning {}: found {} ports ({}ms timeout)\n", &args.host, ports.len(), &args.timeout);
    }

    let mut tasks = Vec::new();

    for i in 0..ports.len() {
        let port = ports[i];
        let host_for_task =args.host.clone();

        tasks.push(tokio::task::spawn(async move{
             scan_port(&host_for_task,port, args.timeout).await
        }));
    }

    let mut statuses: Vec<PortStatus> = Vec::new();

    for handle in tasks{
        let status = handle.await.unwrap();
        statuses.push(status);
    }
    

    if args.json{
        let json_string = to_string_pretty(&statuses).expect("Failed to serialize PortStatus to JSON");
        println!("{:?}", json_string);
    }else{
        let largest_port = ports.iter().max().unwrap();
        let port_width = largest_port.to_string().len();
        let mut open_count: u64 = 0;
        let mut closed_count: u64 = 0;

        for i in 0..statuses.len(){
            let status = &statuses[i];
            let port = status.port;
            let label = if status.is_open {"OPEN"} else {"CLOSED"};

            if status.is_open {
                open_count += 1;
            }else{
                closed_count += 1;
            }

            println!("Port {port:>port_width$}   {label}");
        }

        println!("\nSummary: {} open, {} closed.\n", open_count, closed_count);
    }
}


async fn scan_port(host: &str, port: u16, timeout_ms: u64) -> PortStatus {
    let host_port = format!("{}:{}", host, port);
    let timeout_duration = Duration::from_millis(timeout_ms);

    let stream_future = tokio::time::timeout(timeout_duration, TcpStream::connect(host_port));

    match stream_future.await {
        Ok(inner_result) => match inner_result {
            Ok(_) => {
                return PortStatus {
                    port,
                    is_open: true,
                }
            }
            Err(_) => {
                return PortStatus {
                    port,
                    is_open: false,
                };
            }
        },
        Err(_) => {
            return PortStatus {
                port,
                is_open: false,
            };
        }
    };
}


fn parse_ports(input: &str) -> Result<Vec<u16>, PortParseError> {
    let mut result: Vec<u16> = Vec::new();
    let parts = input.split(",").collect::<Vec<_>>();

    for i in 0..parts.len() {
        let val = parts[i];

        if val.contains("-") {
            let range_parts = val.split("-").collect::<Vec<_>>();
            if range_parts.len() != 2 {
                return Err(PortParseError::InvalidRange);
            }

            let start = match range_parts[0].parse::<u16>() {
                Ok(n) => n,
                Err(_) => return Err(PortParseError::InvalidRange),
            };

            let end = match range_parts[1].parse::<u16>() {
                Ok(n) => n,
                Err(_) => return Err(PortParseError::InvalidRange),
            };

            if start > end {
                return Err(PortParseError::InvalidRange);
            }

            for j in start..=end {
                result.push(j);
            }
        } else {
            let val_num = match val.parse::<u16>() {
                Ok(n) => n,
                Err(_) => return Err(PortParseError::InvalidNumber),
            };

            result.push(val_num);
        }
    }

    result.sort();

    result.dedup();

    Ok(result)
}
