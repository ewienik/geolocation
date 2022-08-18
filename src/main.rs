mod build;
mod check;
mod run;

use {
    clap::{Parser, Subcommand},
    std::path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[clap(value_parser)]
    database: PathBuf,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Check,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Command::Check) => check::check(args.database),
        None => run::run(args.database),
    }
}
