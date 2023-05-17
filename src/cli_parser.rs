use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
pub struct Args {
    #[arg(long = "path", help = "provide path to the program", required = true)]
    pub path: String,
}
