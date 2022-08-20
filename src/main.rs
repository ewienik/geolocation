mod build;
#[cfg(not(feature = "dummy"))]
mod check;
#[cfg(not(feature = "dummy"))]
mod db;
#[cfg(feature = "dummy")]
mod dummy;
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
    #[cfg(not(feature = "dummy"))]
    Check,
}

#[cfg(not(feature = "dummy"))]
use db::{Load, Lookup};
#[cfg(feature = "dummy")]
use dummy::{Load, Lookup};

fn main() {
    let args = Args::parse();
    match args.command {
        #[cfg(not(feature = "dummy"))]
        Some(Command::Check) => check::check(args.database),
        _ => run::run(Load::new(), args.database),
    }
}
