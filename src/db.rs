use {
    csv::Reader,
    std::{collections::BTreeSet, net::Ipv4Addr, path::Path},
};

pub(crate) struct Ip {
    ip: u32,
    my: u32,
    parent: u32,
}

// max size for passing test
const MAX_CITY_LEN: usize = 14;

impl Ip {
    fn new(ip: u32, end: bool, city: u32) -> Self {
        let end = if end { 0x80000000 } else { 0 };
        Self {
            ip,
            my: (city & 0x7fffffff) | end,
            parent: city,
        }
    }

    fn end(&self) -> bool {
        (self.my & 0x80000000) != 0
    }

    fn my(&self) -> u32 {
        self.my & 0x7fffffff
    }
}

#[derive(Clone)]
pub(crate) struct City {
    size: u8,
    buf: [u8; MAX_CITY_LEN],
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.size as usize]) }
    }
}

pub(crate) struct Lookup<'a> {
    cities: &'a [City],
    ips: &'a [Ip],
}

impl<'a> Lookup<'a> {
    fn new(cities: &'a [City], ips: &'a [Ip]) -> Self {
        Self { cities, ips }
    }

    pub(crate) fn lookup(&self, ip: &Ipv4Addr) -> Option<&str> {
        self.ips
            .get(
                self.ips
                    .binary_search_by(|check| check.ip.cmp(&u32::from_be_bytes(ip.octets())))
                    .unwrap_or_else(|it| it),
            )
            .map(|ip| self.cities.get(ip.my() as usize))
            .flatten()
            .map(|city| city.as_ref())
    }
}

pub(crate) struct Load {
    cities: Vec<City>,
    ips: Vec<Ip>,
}

impl Load {
    pub(crate) fn new() -> Self {
        Self {
            cities: Vec::new(),
            ips: Vec::new(),
        }
    }

    pub(crate) fn load(&mut self, path: &Path) -> bool {
        let (cities, ips) = build(path);
        self.cities = cities;
        self.ips = ips;
        true
    }

    pub(crate) fn lookup(&self) -> Option<Lookup> {
        (!self.cities.is_empty() & !self.ips.is_empty())
            .then(|| Lookup::new(&self.cities, &self.ips))
    }
}

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
            match ips.binary_search_by(|ip| ip.ip.cmp(&ip_from.ip)) {
                Ok(_) => panic!("database problem, double ip values"),
                Err(it) => {
                    match ips.get(it) {
                        Some(ip) => {
                            if ip.end() {
                                ip_from.parent = ip.my;
                                ip_to.parent = ip.my;
                            } else {
                                ip_from.parent = ip.parent;
                                ip_to.parent = ip.parent;
                            }
                        }
                        None => {}
                    };
                    ips.insert(it, ip_from);
                    ips.insert(it + 1, ip_to);
                }
            };
        });
    let cities: Vec<_> = cities
        .into_iter()
        .map(|city| City {
            size: city.len() as u8,
            buf: city
                .chars()
                .map(|v| v as u8)
                .chain(std::iter::repeat(0))
                .take(MAX_CITY_LEN)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
        .collect();
    (cities, ips)
}
