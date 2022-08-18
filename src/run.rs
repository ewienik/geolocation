use std::{io, net::Ipv4Addr, path::PathBuf, str::FromStr};

fn load(_path: PathBuf) -> bool {
    let line = if let Some(line) = io::stdin().lines().map(|l| l.unwrap()).next() {
        line
    } else {
        return false;
    };
    let result = match line.as_str() {
        "LOAD" => true,
        "EXIT" => false,
        _ => panic!("UNKNOWN COMMAND"),
    };
    println!("OK");
    result
}

fn lookup() {
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
            if !line.starts_with("LOOKUP") {
                panic!("UNKNOWN COMMAND");
            }
            if !line.starts_with("LOOKUP ") {
                println!("ERR");
                return;
            }
            match Ipv4Addr::from_str(&line[7..]) {
                Ok(addr) => println!("{addr:?}"),
                Err(_) => println!("ERR"),
            }
        });
}

pub(crate) fn run(path: PathBuf) {
    println!("READY");
    if !load(path) {
        return;
    }
    lookup();
}
