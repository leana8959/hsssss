mod ascii_animation;
mod cli_parser;
mod telnet_parser;

use clap::Parser;
use cli_parser::Args;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::select;
use tokio::time;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use ascii_animation::AsciiAnimation;
use telnet_parser::TelnetParser;

macro_rules! err_break {
    ($e:expr) => {
        if let Err(_) = $e {
            break;
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let listener = TcpListener::bind(args.address).await?;
    let backing_buffer = fs::read_to_string(args.path).expect("should have a frames file");
    let shared_buffer: Arc<str> = backing_buffer.into();

    loop {
        let buffer = Arc::clone(&shared_buffer);

        let (mut socket, addr) = listener.accept().await?;

        println!("Connection from: {}", addr);

        tokio::spawn(async move {
            // TODO: limit connection time
            let mut parser = TelnetParser::new();
            let mut animation = AsciiAnimation::new(&buffer);

            let mut buf = [0; 1024];
            let mut interval = time::interval(Duration::from_millis(100));

            loop {
                parser.clear();

                select! {
                    value = socket.read(&mut buf) => {
                        if let Ok(n) = value {
                            parser.read_codes(&buf[..n]);
                            err_break!(socket.write_all(parser.respond()).await);
                        }
                    }

                    _ = interval.tick() => {
                        animation.set_width(parser.width());
                        animation.set_height(parser.height());
                        err_break!(
                            socket
                                .write_all(format!("\x1bc{}\nHit ^C to exit", animation.next_frame()).as_bytes())
                                .await
                        )
                    }
                }

                if parser.exit_now()
                    || Instant::now().duration_since(parser.created()) > Duration::from_secs(5)
                {
                    err_break!(
                        socket
                            .write_all(
                                b"\nByeee!\nLearn more: https://git.earth2077.fr/leana/hsssss\n"
                            )
                            .await
                    );
                    socket.shutdown().await.unwrap();
                    break;
                };
            }

            println!("Closing connection from: {}", addr);
        });
    }
}
