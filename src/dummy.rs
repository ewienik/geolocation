use std::{net::Ipv4Addr, path::Path, str::FromStr};

pub(crate) struct Load {}

impl Load {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn load(&self, _path: &Path) -> bool {
        true
    }

    pub(crate) fn lookup(&self) -> Option<Lookup> {
        Some(Lookup::new())
    }
}

pub(crate) struct Lookup {
    data: Vec<(Ipv4Addr, &'static str)>,
}

impl Lookup {
    fn new() -> Self {
        Self {
            data: [
                (Ipv4Addr::from_str("1.0.0.0").unwrap(), "US,Los Angeles"),
                (Ipv4Addr::from_str("71.6.28.0").unwrap(), "US,San Jose"),
                (Ipv4Addr::from_str("71.6.28.255").unwrap(), "US,San Jose"),
                (Ipv4Addr::from_str("71.6.29.0").unwrap(), "US,Concord"),
                (Ipv4Addr::from_str("53.103.144.0").unwrap(), "DE,Stuttgart"),
                (
                    Ipv4Addr::from_str("53.255.255.255").unwrap(),
                    "DE,Stuttgart",
                ),
                (Ipv4Addr::from_str("54.0.0.0").unwrap(), "US,Rahway"),
                (
                    Ipv4Addr::from_str("223.255.255.255").unwrap(),
                    "AU,Brisbane",
                ),
                (Ipv4Addr::from_str("5.44.16.0").unwrap(), "GB,Hastings"),
                (Ipv4Addr::from_str("8.24.99.0").unwrap(), "US,Hastings"),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub(crate) fn lookup(&self, ip: &Ipv4Addr) -> Option<&str> {
        self.data
            .iter()
            .find(|(check, _)| check == ip)
            .map(|(_, v)| *v)
    }
}
