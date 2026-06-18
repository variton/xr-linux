//! Minimal TCP client command-line application.
//!
//! The client connects to a remote TCP endpoint, sends a user-provided text
//! message, closes the write side of the stream, then prints the full response
//! returned by the server.

use clap::Parser;
use std::io::Write;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Hostname or IP address of the TCP server.
    #[arg(long, short)]
    host: String,

    /// TCP port exposed by the server.
    #[arg(long, short)]
    port: u16,

    /// Command to execute remotely.
    #[arg(required = true)]
    message: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let command = args.message.join(" ");

    let mut stream = TcpStream::connect(&addr).await?;

    // Send command terminated by newline.
    stream.write_all(command.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Tell the server we're done sending.
    stream.shutdown().await?;

    // Stream output as it arrives.
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        print!("{}", String::from_utf8_lossy(&buf[..n]));
        std::io::stdout().flush()?;
    }

    Ok(())
}
