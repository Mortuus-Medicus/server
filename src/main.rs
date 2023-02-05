use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncReadExt;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    match listener.accept().await {
        Ok((mut socket, addr)) => {
            println!("new client: {:?}", addr);
            let mut msg = [0; 12];
            socket.read_exact(&mut msg).await?;
            println!("{}", std::str::from_utf8(&msg).unwrap());
        },
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    Ok(())
}