use {
    crate::db::Db,
    std::{
        io::{self, Write},
        net::Ipv4Addr,
        path::PathBuf,
        str::FromStr,
    },
};

pub(crate) fn run(db: &mut impl Db, path: PathBuf) {
    println!("READY");
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
            if line == "LOAD" {
                println!(
                    "{}",
                    match db.load(&path) {
                        Ok(()) => "OK",
                        Err(_) => "ERR",
                    }
                );
                io::stdout().flush().unwrap();
                return;
            }
            if !line.starts_with("LOOKUP") {
                panic!("UNKNOWN COMMAND");
            }
            if !line.starts_with("LOOKUP ") {
                println!("ERR");
                return;
            }
            match Ipv4Addr::from_str(&line[7..]) {
                Ok(addr) => println!(
                    "{}",
                    match db.lookup(&addr) {
                        Ok(result) => result,
                        Err(_) => "ERR",
                    }
                ),
                Err(_) => println!("ERR"),
            }
            io::stdout().flush().unwrap();
        });
}
