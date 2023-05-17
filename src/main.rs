mod telnet_parser;
use telnet_parser::TelnetParser;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:23").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut parser = TelnetParser::new();
            let mut buf = [0; 1024];
            while !parser.exit_now() {
                parser.clear();

                match socket.read(&mut buf).await {
                    // HACK: remove the unwraps
                    Ok(n) => {
                        parser.read_codes(&buf[..n]);
                        socket.write(parser.respond()).await.unwrap();
                        socket
                            .write(
                                format!("width: {} height: {}\n", parser.width(), parser.height())
                                    .as_bytes(),
                            )
                            .await
                            .unwrap();
                    }
                    Err(_) => (),
                };

                if parser.exit_now() {
                    socket.write(b"Byeeeee #Clawthorn #TOH\n").await.unwrap();
                };
            }
        });
    }
}
