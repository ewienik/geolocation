use {
    anyhow::Result,
    std::{net::Ipv4Addr, path::Path},
};

pub(crate) trait Db {
    fn load(&mut self, path: &Path) -> Result<()>;
    fn lookup(&self, ip: &Ipv4Addr) -> Result<&str>;
}
