use clap::Parser;
use std::{io, process::Stdio, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    process::Command,
    sync::Mutex,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    host: String,

    #[arg(long, short)]
    port: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (socket, peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = handle_client(socket).await {
                eprintln!("Error handling {peer_addr}: {err}");
            }
        });
    }
}

async fn handle_client(socket: TcpStream) -> io::Result<()> {
    let (read_half, write_half) = socket.into_split();

    let mut reader = BufReader::new(read_half);
    let writer = Arc::new(Mutex::new(write_half));

    let mut command = String::new();
    reader.read_line(&mut command).await?;

    let command = command.trim();

    if command.is_empty() {
        let mut w = writer.lock().await;
        w.write_all(b"No command received\n").await?;
        w.shutdown().await?;
        return Ok(());
    }

    run_and_stream(command, writer.clone()).await?;

    let mut w = writer.lock().await;
    w.shutdown().await?;

    Ok(())
}

async fn run_and_stream(
    command: &str,
    writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
) -> io::Result<()> {
    let mut child = Command::new("stdbuf")
        .arg("-oL")
        .arg("-eL")
        .arg("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_writer = writer.clone();
    let stdout_task = tokio::spawn(async move { stream_output(stdout, stdout_writer).await });

    let stderr_writer = writer.clone();
    let stderr_task = tokio::spawn(async move { stream_output(stderr, stderr_writer).await });

    let status = child.wait().await?;

    stdout_task.await??;
    stderr_task.await??;

    if !status.success() {
        let msg = match status.code() {
            Some(code) => format!("\nCommand exited with status code {code}\n"),
            None => "\nCommand terminated by signal\n".to_string(),
        };

        let mut w = writer.lock().await;
        w.write_all(msg.as_bytes()).await?;
    }

    Ok(())
}

async fn stream_output<R>(
    mut reader: R,
    writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
) -> io::Result<()>
where
    R: AsyncReadExt + Unpin,
{
    let mut buf = [0u8; 4096];

    loop {
        let n = reader.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let mut w = writer.lock().await;
        w.write_all(&buf[..n]).await?;
        w.flush().await?;
    }

    Ok(())
}
