use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    console_subscriber::init();
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut read, mut write) = io::split(socket);
            let result = io::copy(&mut read, &mut write).await;

            match result {
                Ok(copied) => {
                    eprintln!("Copied bytes: {}", copied);
                }
                Err(e) => {
                    eprintln!("failed to copy {}", e);
                }
            }
        });
    }
}
