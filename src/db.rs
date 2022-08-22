use std::{cmp, fs::File, io::Read, mem, net::Ipv4Addr, path::Path, slice};

#[derive(Clone)]
pub(crate) struct Ip {
    ip: u32,
    my: u32,
    parent: u32,
}

// max size for passing test
const MAX_CITY_LEN: usize = 14;

impl Ip {
    #[allow(dead_code)]
    pub(crate) fn new(ip: u32, end: bool, city: u32) -> Self {
        let end = if end { 0x80000000 } else { 0 };
        Self {
            ip,
            my: (city & 0x7fffffff) | end,
            parent: city,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn ip(&self) -> u32 {
        self.ip
    }

    #[allow(dead_code)]
    pub(crate) fn end(&self) -> bool {
        (self.my & 0x80000000) != 0
    }

    #[allow(dead_code)]
    pub(crate) fn my(&self) -> u32 {
        self.my & 0x7fffffff
    }

    #[allow(dead_code)]
    pub(crate) fn parent(&self) -> u32 {
        self.parent
    }

    #[allow(dead_code)]
    pub(crate) fn set_parent(&mut self, city: u32) {
        self.parent = city;
    }
}

#[derive(Clone)]
pub(crate) struct City {
    size: u8,
    buf: [u8; MAX_CITY_LEN],
}

impl City {
    #[allow(dead_code)]
    pub(crate) fn new(city: &str) -> Self {
        Self {
            size: cmp::min(MAX_CITY_LEN, city.len()) as u8,
            buf: city
                .chars()
                .map(|v| v as u8)
                .chain(std::iter::repeat(0))
                .take(MAX_CITY_LEN)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
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

    #[allow(dead_code)]
    pub(crate) fn lookup(&self, ip: &Ipv4Addr) -> Option<&str> {
        self.ips
            .get(
                self.ips
                    .binary_search_by(|check| check.ip.cmp(&u32::from_be_bytes(ip.octets())))
                    .unwrap_or_else(|it| it),
            )
            .and_then(|ip| self.cities.get(ip.my() as usize))
            .map(|city| city.as_ref())
    }
}

pub(crate) struct Load {
    cities: Vec<City>,
    ips: Vec<Ip>,
}

impl Load {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            cities: Vec::new(),
            ips: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn load(&mut self, path: &Path) -> Option<()> {
        let mut db = File::open(path).ok()?;
        let mut read_size = || {
            let mut usize_buf = [0u8; mem::size_of::<usize>()];
            (db.read(&mut usize_buf).unwrap() == mem::size_of::<usize>())
                .then(|| usize::from_le_bytes(usize_buf))
        };
        let cities_size = read_size()?;
        let ips_size = read_size()?;
        self.cities = vec![City::new(""); cities_size];
        self.ips = vec![Ip::new(0, false, 0); ips_size];
        let cities_bytes = unsafe {
            let ptr = mem::transmute::<*mut City, *mut u8>(self.cities.as_mut_ptr());
            slice::from_raw_parts_mut(ptr, cities_size * mem::size_of::<City>())
        };
        let ips_bytes = unsafe {
            let ptr = mem::transmute::<*mut Ip, *mut u8>(self.ips.as_mut_ptr());
            slice::from_raw_parts_mut(ptr, ips_size * mem::size_of::<Ip>())
        };
        (db.read(cities_bytes).unwrap() == self.cities.len() * mem::size_of::<City>())
            .then(|| {})?;
        (db.read(ips_bytes).unwrap() == self.ips.len() * mem::size_of::<Ip>()).then(|| {})?;
        Some(())
    }

    #[allow(dead_code)]
    pub(crate) fn lookup(&self) -> Option<Lookup> {
        (!self.cities.is_empty() & !self.ips.is_empty())
            .then(|| Lookup::new(&self.cities, &self.ips))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ip() {
        let mut ip = Ip::new(100, false, 20);
        assert_eq!(ip.ip(), 100);
        assert_eq!(ip.end(), false);
        assert_eq!(ip.my(), 20);
        assert_eq!(ip.parent(), 20);
        ip.set_parent(30);
        assert_eq!(ip.ip(), 100);
        assert_eq!(ip.end(), false);
        assert_eq!(ip.my(), 20);
        assert_eq!(ip.parent(), 30);

        let mut ip = Ip::new(100, true, 20);
        assert_eq!(ip.ip(), 100);
        assert_eq!(ip.end(), true);
        assert_eq!(ip.my(), 20);
        assert_eq!(ip.parent(), 20);
        ip.set_parent(30);
        assert_eq!(ip.ip(), 100);
        assert_eq!(ip.end(), true);
        assert_eq!(ip.my(), 20);
        assert_eq!(ip.parent(), 30);
    }

    #[test]
    fn new_city() {
        let city = City::new("");
        assert_eq!(city.size, 0);
        let city = City::new("a");
        assert_eq!(city.size, 1);
        assert_eq!(city.as_ref(), "a");
        let city = City::new("01234567890123");
        assert_eq!(city.size, 14);
        assert_eq!(city.as_ref(), "01234567890123");
        let city = City::new("123456789012345");
        assert_eq!(city.size, 14);
        assert_eq!(city.as_ref(), "12345678901234");
    }
}
