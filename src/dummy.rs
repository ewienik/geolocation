use {
    crate::db::Db,
    anyhow::Result,
    std::{net::Ipv4Addr, path::Path, str::FromStr},
};

pub(crate) struct Dummy<'a> {
    data: Vec<(Ipv4Addr, &'a str)>,
}

impl Dummy<'_> {
    pub(crate) fn new() -> Self {
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
}

impl Db for Dummy<'_> {
    fn load(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn lookup(&self, ip: &Ipv4Addr) -> Result<&str> {
        self.data
            .iter()
            .find(|(check, _)| check == ip)
            .map(|(_, v)| *v)
            .ok_or(anyhow::anyhow!("not found"))
    }
}
