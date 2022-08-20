use {
    crate::{Load, Lookup},
    std::{
        io::{self, Write},
        net::Ipv4Addr,
        path::{Path, PathBuf},
        str::FromStr,
    },
};

fn load(db: &mut Load, path: &Path) -> bool {
    io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|line| match line.as_str() {
            "EXIT" => {
                println!("OK");
                false
            }
            _ => true,
        })
        .find_map(|line| {
            match line.as_str() {
                "LOAD" => {
                    if db.load(&path).is_some() {
                        return Some(());
                    }
                }
                _ => {}
            };
            println!("ERR");
            None
        })
        .is_some()
}

fn lookup(dblookup: &Lookup) {
    io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|line| match line.as_str() {
            "EXIT" => {
                println!("OK");
                false
            }
            _ => true,
        })
        .for_each(|line| {
            if !line.starts_with("LOOKUP ") {
                println!("ERR");
                return;
            }
            println!(
                "{}",
                match Ipv4Addr::from_str(&line[7..]) {
                    Ok(addr) => dblookup.lookup(&addr).unwrap_or("ERR"),
                    Err(_) => "ERR",
                }
            );
            io::stdout().flush().unwrap();
        });
}

pub(crate) fn run(mut dbload: Load, path: PathBuf) {
    println!("READY");
    if load(&mut dbload, &path) {
        let dblookup = dbload.lookup().unwrap();
        println!("OK");
        io::stdout().flush().unwrap();
        lookup(&dblookup);
    };
}
