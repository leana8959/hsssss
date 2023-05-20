use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
pub struct Args {
    #[arg(long = "path", help = "provide path to the program", required = true)]
    pub path: String,

    #[arg(
        long = "addr",
        help = "specify which address to bind to. format: [addr:port]",
        required = true
    )]
    pub address: String,
}
