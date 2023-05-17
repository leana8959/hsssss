mod ascii_animation;
mod telnet_parser;
use std::fs;
use std::{thread::sleep, time::Duration};
use tokio::time::{self, interval};

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

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut parser = TelnetParser::new();
            let mut buf = [0; 1024];

            // HACK: don't read the file multiple times plz
            let backing_buffer = fs::read_to_string("frames.txt").unwrap();
            let mut animation = AsciiAnimation::new(&backing_buffer);
            let mut interval = time::interval(Duration::from_millis(50));

            while !parser.exit_now() {
                parser.clear();

                select! {
                    value = socket.read(&mut buf) => {
                        match value {
                            // HACK: remove the unwraps
                            Ok(n) => {
                                // Protocol stuff
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
                        socket.write(b"\x1bc").await.unwrap();
                        socket
                            .write(animation.next_frame().as_bytes())
                            .await
                            .unwrap();
                    }
                }

                if parser.exit_now() {
                    socket.write(b"Byeeeee #Clawthorn #TOH\n").await.unwrap();
                };
            }
        });
    }
}
