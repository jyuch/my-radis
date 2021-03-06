use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut read, mut write) = io::split(socket);

    let _write_task = tokio::spawn(async move {
        write.write_all(b"hello\r\n").await?;
        write.write_all(b"world\r\n").await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];
    let mut received = 0;

    loop {
        let n = read.read(&mut buf).await?;
        received += n;

        println!("GOT {:?}", &buf[..n]);

        if received <= 14 {
            break;
        }
    }

    Ok(())
}
