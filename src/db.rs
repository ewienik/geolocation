use std::{fs::File, io::Read, mem, net::Ipv4Addr, path::Path, slice};

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
            size: city.len() as u8,
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
            .map(|ip| self.cities.get(ip.my() as usize))
            .flatten()
            .map(|city| city.as_ref())
    }
}

pub(crate) struct Load {
    cities: Vec<u8>,
    ips: Vec<u8>,
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
        self.cities = vec![0u8; cities_size * mem::size_of::<City>()];
        self.ips = vec![0u8; ips_size * mem::size_of::<Ip>()];
        (db.read(&mut self.cities).unwrap() == self.cities.len()).then(|| {})?;
        (db.read(&mut self.ips).unwrap() == self.ips.len()).then(|| {})?;
        Some(())
    }

    #[allow(dead_code)]
    pub(crate) fn lookup(&self) -> Option<Lookup> {
        (!self.cities.is_empty() & !self.ips.is_empty()).then(|| {
            let cities = unsafe {
                let ptr = mem::transmute::<*const u8, *const City>(self.cities.as_ptr());
                slice::from_raw_parts(ptr, self.cities.len() / mem::size_of::<City>())
            };
            let ips = unsafe {
                let ptr = mem::transmute::<*const u8, *const Ip>(self.ips.as_ptr());
                slice::from_raw_parts(ptr, self.ips.len() / mem::size_of::<Ip>())
            };
            Lookup::new(cities, ips)
        })
    }
}
