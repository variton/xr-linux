//! Minimal TCP client command-line application.
//!
//! The client connects to a remote TCP endpoint, sends a user-provided text
//! message, closes the write side of the stream, then prints the full response
//! returned by the server.

use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Command-line arguments for the TCP client.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Hostname or IP address of the TCP server.
    #[arg(long, short)]
    host: String,

    /// TCP port exposed by the server.
    #[arg(long, short)]
    port: u16,

    /// Message words to send, joined with spaces before transmission.
    #[arg(required = true)]
    message: Vec<String>,
}

/// Connects to the configured TCP server, sends the message, and prints the response.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let message = args.message.join(" ");

    let mut stream = TcpStream::connect(&addr).await?;

    // Closing the write side signals to the server that the request is complete.
    stream.write_all(message.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}
