mod ascii_animation;
mod cli_parser;
mod telnet_parser;

use clap::Parser;
use cli_parser::Args;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::time;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use ascii_animation::AsciiAnimation;
use telnet_parser::TelnetParser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let listener = TcpListener::bind(format!("{}:23", args.address)).await?;
    let backing_buffer = fs::read_to_string(args.path).expect("should have a frames file");
    let shared_buffer: Arc<str> = backing_buffer.into();

    loop {
        let buffer = Arc::clone(&shared_buffer);

        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut parser = TelnetParser::new();
            let mut animation = AsciiAnimation::new(&buffer);

            let mut buf = [0; 1024];
            let mut interval = time::interval(Duration::from_millis(100));

            loop {
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
                        animation.set_width(parser.width());
                        animation.set_height(parser.height());
                        socket
                            .write(format!("\x1bc{}\nHit ^C to exit", animation.next_frame()).as_bytes())
                            .await
                            .expect("should send next frame");
                    }
                }

                if parser.exit_now() {
                    socket.write(b"\nByeee!\nLearn more: https://git.earth2077.fr/leana/hsssss\n").await.unwrap();
                    return;
                };
            }
        });
    }
}
