mod db;
mod run;

use {
    db::{Load, Lookup},
    std::{env, path::PathBuf, process},
};

fn main() {
    let database = {
        let mut args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("usage: args[0] database_path");
            process::exit(1);
        }
        PathBuf::from(args.remove(1))
    };
    run::run(Load::new(), database);
}
