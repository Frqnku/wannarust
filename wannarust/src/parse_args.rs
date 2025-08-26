use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub reverse: bool,
}

pub fn parse_args() -> Args {
	Args::parse()
}