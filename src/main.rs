mod build;
mod check;
mod db;
#[cfg(feature = "dummy")]
mod dummy;
#[cfg(not(feature = "dummy"))]
mod ranges;
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
    #[cfg(feature = "dummy")]
    let mut db = dummy::Dummy::new();
    #[cfg(not(feature = "dummy"))]
    let mut db = ranges::Ranges::new();
    match args.command {
        Some(Command::Check) => check::check(args.database),
        None => run::run(&mut db, args.database),
    }
}
