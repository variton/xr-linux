use clap::Parser;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    process::Command,
};

/// Command-line configuration for the TCP command server.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Hostname or IP address to bind to
    #[arg(long, short)]
    host: String,

    /// TCP port to listen on
    #[arg(long, short)]
    port: u16,
}

/// Starts the TCP listener and spawns one async task per client connection.
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr).await?;

    // println!("Server listening on {addr}");

    loop {
        let (socket, peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = handle_client(socket).await {
                eprintln!("Error handling {peer_addr}: {err}");
            }
        });
    }
}

/// Reads a complete command from a client, executes it, and returns the result.
async fn handle_client(mut socket: TcpStream) -> io::Result<()> {
    let mut buffer = Vec::new();

    socket.read_to_end(&mut buffer).await?;

    let message = String::from_utf8_lossy(&buffer).trim().to_string();

    if message.is_empty() {
        socket.write_all(b"No command received\n").await?;
        socket.shutdown().await?;
        return Ok(());
    }

    // println!("Executing: {message}");

    let output = run_command(&message).await?;

    if output.is_empty() {
        socket
            .write_all(b"Command completed with no output\n")
            .await?;
    } else {
        socket.write_all(output.as_bytes()).await?;
    }

    socket.shutdown().await?;

    Ok(())
}

/// Executes a shell command and returns stdout when the command succeeds.
///
/// The command is passed to `sh -c`, so callers must only provide trusted input.
async fn run_command(command: &str) -> io::Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).output().await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("command failed: {stderr}"),
        ))
    }
}
