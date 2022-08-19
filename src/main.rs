mod build;
mod check;
mod db;
mod dummy;
mod run;

use {
    clap::{Parser, Subcommand},
    dummy::Dummy,
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
    let mut db = Dummy::new();
    match args.command {
        Some(Command::Check) => check::check(args.database),
        None => run::run(&mut db, args.database),
    }
}
