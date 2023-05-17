mod ascii_animation;
mod telnet_parser;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

use ascii_animation::AsciiAnimation;
use telnet_parser::TelnetParser;

use tokio::select;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:23").await?;

    // TODO: make this modular (cli option parser?)
    let backing_buffer = fs::read_to_string("frames.txt").expect("should have a frames file");
    let shared_buffer = Arc::new(backing_buffer);

    loop {
        let buffer = Arc::clone(&shared_buffer);

        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut parser = TelnetParser::new();
            let mut animation = AsciiAnimation::new(&buffer);

            let mut buf = [0; 1024];
            let mut interval = time::interval(Duration::from_millis(50));

            while !parser.exit_now() {
                parser.clear();

                select! {
                    value = socket.read(&mut buf) => {
                        match value {
                            Ok(n) => {
                                parser.read_codes(&buf[..n]);
                                socket
                                    .write(parser.respond())
                                    .await
                                    .expect("should write the response");
                            }
                            Err(_) => (),
                        }
                    }

                    _ = interval.tick() => {
                        animation.set_width(parser.width() as usize);
                        socket
                            .write(b"\x1bc")
                            .await
                            .expect("should clear screen");
                        socket
                            .write(animation.next_frame().as_bytes())
                            .await
                            .expect("should send next frame");
                    }
                }

                if parser.exit_now() {
                    socket.write(b"Byeeeee #Clawthorn #TOH\n").await.unwrap();
                };
            }
        });
    }
}
