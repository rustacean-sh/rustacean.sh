mod cmd;

use anyhow::Result;
use clap::Parser;

use self::cmd::generate::GenerateOpt;

#[derive(Debug, Parser)]
#[command(name = "rustacean", about = "Rustacean.sh CLI", next_line_help = true)]
pub enum Command {
    /// Generates the database static files
    Generate(GenerateOpt),
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Generate(opt) => opt.exec(),
    }
}
