use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX_PORT: u16 = 65535;

// Options struct for CLI arguments.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
struct Options {
    #[bpaf(short)]
    switch: bool,
}

// Arguments struct for IP address and port specification.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, argument("Address"))]
    /// The address that you want to scan. Must be a valid IPv4 address.
    pub address: IpAddr,
    
    #[bpaf(long, short('p'), argument("Port"))]
    /// The port(s) to scan. Specify a single port (e.g., 80), a list of ports (e.g., 80,22,443,8080),
    /// or a range of ports (e.g., 1-1024).
    pub ports: String,
}

// Function to validate and parse the port specification.
fn parse_ports(ports: &str) -> Vec<u16> {
    let mut result = Vec::new();

    for part in ports.split(',') {
        if part.contains('-') {
            // Handle port range (e.g., "1-1024").
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    u16::from_str(range[0]),
                    u16::from_str(range[1]),
                ) {
                    for port in start..=end {
                        result.push(port);
                    }
                }
            }
        } else if let Ok(port) = u16::from_str(part) {
            // Handle individual ports (e.g., "80").
            result.push(port);
        }
    }

    result
}

// Function to perform port scan on a single port.
async fn scan_port(port_sender: Sender<u16>, port: u16, addr: IpAddr) {
    // Print a message indicating the scanning of the current port.
    // println!("Scanning {}:{}...", addr, port);

    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            // If the connection is successful, print a dot and send the open port.
            print!(".");
            io::stdout().flush().unwrap();
            port_sender.send(port).unwrap();
        }
        Err(_) => {
            //pass
        }
    }
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments.
    let opts = arguments().run();

    // Parse the specified ports.
    let ports_to_scan = parse_ports(&opts.ports);
    
    println!("Scanning {} ports {} ...", &opts.address, &opts.ports);

    // Create a channel for sending open port numbers.
    let (port_sender, port_receiver) = channel();

    // Start tasks to scan ports in the specified range concurrently.
    for port in ports_to_scan {
        let port_sender = port_sender.clone();
        task::spawn(async move { scan_port(port_sender, port, opts.address).await });
    }

    // Close the sender to signal that no more ports will be sent.
    drop(port_sender);

    // Collect open ports and sort them.
    let mut open_ports = Vec::new();
    for port in port_receiver {
        open_ports.push(port);
    }

    println!("\nOpen Ports:");
    open_ports.sort();
    for port in open_ports {
        println!("Port {} is open", port);
    }
}
