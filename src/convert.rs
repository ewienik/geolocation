mod db;

use {
    csv::Reader,
    db::{City, Ip},
    std::{
        collections::BTreeSet,
        env,
        fs::File,
        io::Write,
        mem,
        path::{Path, PathBuf},
        process, slice,
    },
};

fn build(path: &Path) -> (Vec<City>, Vec<Ip>) {
    let cities: BTreeSet<_> = Reader::from_path(&path)
        .unwrap()
        .records()
        .map(|record| record.unwrap())
        .map(|record| format!("{},{}", record.get(2).unwrap(), record.get(5).unwrap()))
        .collect();
    let cities: Vec<_> = cities.into_iter().collect();
    let mut ips: Vec<Ip> = Vec::new();
    Reader::from_path(&path)
        .unwrap()
        .records()
        .map(|record| record.unwrap())
        .map(|record| {
            (
                record.get(0).unwrap().parse::<u32>().unwrap(),
                record.get(1).unwrap().parse::<u32>().unwrap(),
                format!("{},{}", record.get(2).unwrap(), record.get(5).unwrap(),),
            )
        })
        .map(|(ip_from, ip_to, city)| (ip_from, ip_to, cities.binary_search(&city).unwrap() as u32))
        .map(|(ip_from, ip_to, city)| (Ip::new(ip_from, false, city), Ip::new(ip_to, true, city)))
        .for_each(|(mut ip_from, mut ip_to)| {
            match ips.binary_search_by(|ip| ip.ip().cmp(&ip_from.ip())) {
                Ok(_) => panic!("database problem, double ip values"),
                Err(it) => {
                    match ips.get(it) {
                        Some(ip) => {
                            if ip.end() {
                                ip_from.set_parent(ip.my());
                                ip_to.set_parent(ip.my());
                            } else {
                                ip_from.set_parent(ip.parent());
                                ip_to.set_parent(ip.parent());
                            }
                        }
                        None => {}
                    };
                    ips.insert(it, ip_from);
                    ips.insert(it + 1, ip_to);
                }
            };
        });
    let cities: Vec<_> = cities.into_iter().map(|city| City::new(&city)).collect();
    (cities, ips)
}

fn main() {
    let (db_in, db_out) = {
        let mut args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            println!("usage: args[0] in_database_path out_database_path");
            process::exit(1);
        }
        (PathBuf::from(args.remove(1)), PathBuf::from(args.remove(1)))
    };
    let (cities, ips) = build(&db_in);
    let cities_size = cities.len();
    let ips_size = ips.len();
    let cities_bytes = unsafe {
        let ptr = mem::transmute::<*const City, *const u8>(cities.as_ptr());
        slice::from_raw_parts(ptr, cities_size * mem::size_of::<City>())
    };
    let ips_bytes = unsafe {
        let ptr = mem::transmute::<*const Ip, *const u8>(ips.as_ptr());
        slice::from_raw_parts(ptr, ips_size * mem::size_of::<Ip>())
    };
    let mut db_out = File::create(&db_out).unwrap();
    assert_eq!(
        db_out.write(&cities_size.to_le_bytes()).unwrap(),
        mem::size_of::<usize>()
    );
    assert_eq!(
        db_out.write(&ips_size.to_le_bytes()).unwrap(),
        mem::size_of::<usize>()
    );
    assert_eq!(db_out.write(cities_bytes).unwrap(), cities_bytes.len());
    assert_eq!(db_out.write(ips_bytes).unwrap(), ips_bytes.len());
}
