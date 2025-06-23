mod cli;
mod tak_board;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_name = "PORT")]
    listen: Option<i32>,
}

fn main() {
    let args = Args::parse();
    match args.listen {
        None => cli::start_game(),
        Some(port) => todo!("make the server to listen on"),
    }
}
