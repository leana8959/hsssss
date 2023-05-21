mod ascii_animation;
mod cli_parser;
mod telnet_parser;

use clap::Parser;
use cli_parser::Args;
use std::collections::HashSet;
use std::fs;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
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
    let connected: Arc<Mutex<HashSet<IpAddr>>> = Arc::new(Mutex::new(HashSet::new()));

    loop {
        let buffer = Arc::clone(&shared_buffer);
        let connected = Arc::clone(&connected);

        let (mut socket, addr) = listener.accept().await?;

        if connected.lock().unwrap().contains(&addr.ip()) {
            println!("Blocked {addr} from connecting");
            continue;
        }

        connected.lock().unwrap().insert(addr.ip());

        println!("Connection from: {}", addr);

        tokio::spawn(async move {
            let mut parser = TelnetParser::new();
            let mut animation = AsciiAnimation::new(&buffer);

            let mut buf = [0; 1024];
            let mut interval = time::interval(Duration::from_millis(100));
            let created = Instant::now();

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
                    || Instant::now().duration_since(created) > Duration::from_secs(60)
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

            // end of `spawn` block
            println!("Closing connection from: {}", addr);
            connected.lock().unwrap().remove(&addr.ip());
        });
    }
}
