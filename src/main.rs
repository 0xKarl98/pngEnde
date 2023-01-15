mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

//dyn is a shorthand for dynamic
//it can refer to any type that implements the "Error" trait 
pub type Error = Box<dyn std::error::Error>;


pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::from_args();
    commands::run(cli.subcommand) 
}