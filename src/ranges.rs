use {
    crate::db::Db,
    anyhow::Result,
    csv::Reader,
    std::{collections::BTreeSet, net::Ipv4Addr, path::Path},
};

struct Ip {
    ip: Ipv4Addr,
    end: bool,
    my: usize,
    parent: usize,
}

pub(crate) struct Ranges {
    cities: Vec<String>,
    ips: Vec<Ip>,
}

impl Ranges {
    pub(crate) fn new() -> Self {
        Self {
            cities: Vec::new(),
            ips: Vec::new(),
        }
    }
}

impl Db for Ranges {
    fn load(&mut self, path: &Path) -> Result<()> {
        let cities: BTreeSet<_> = Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|record| record.unwrap())
            .map(|record| format!("{},{}", record.get(2).unwrap(), record.get(5).unwrap()))
            .collect();
        self.cities = cities.into_iter().collect();
        self.ips.clear();
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
            .map(|(ip_from, ip_to, city)| (Ipv4Addr::from(ip_from), Ipv4Addr::from(ip_to), city))
            .map(|(ip_from, ip_to, city)| {
                (ip_from, ip_to, self.cities.binary_search(&city).unwrap())
            })
            .map(|(ip_from, ip_to, city)| {
                (
                    Ip {
                        ip: ip_from,
                        end: false,
                        my: city,
                        parent: city,
                    },
                    Ip {
                        ip: ip_to,
                        end: true,
                        my: city,
                        parent: city,
                    },
                )
            })
            .for_each(|(mut ip_from, mut ip_to)| {
                match self.ips.binary_search_by(|ip| ip.ip.cmp(&ip_from.ip)) {
                    Ok(_) => panic!("database problem, double ip values"),
                    Err(it) => {
                        match self.ips.get(it) {
                            Some(ip) => {
                                if ip.end {
                                    ip_from.parent = ip.my;
                                    ip_to.parent = ip.my;
                                } else {
                                    ip_from.parent = ip.parent;
                                    ip_to.parent = ip.parent;
                                }
                            }
                            None => {}
                        };
                        self.ips.insert(it, ip_from);
                        self.ips.insert(it + 1, ip_to);
                    }
                };
            });
        Ok(())
    }

    fn lookup(&self, ip: &Ipv4Addr) -> Result<&str> {
        self.ips
            .get(
                self.ips
                    .binary_search_by(|check| check.ip.cmp(&ip))
                    .unwrap_or_else(|it| it),
            )
            .map(|ip| self.cities.get(ip.my))
            .flatten()
            .map(|city| city.as_str())
            .ok_or(anyhow::anyhow!("not found"))
    }
}
