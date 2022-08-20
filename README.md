# geolocation

Technology preview for lookup in geolocation db.

## binaries

- geoloc: main app for lookup ip in preprocessed db:
  `geoloc preprocessed-database-path`
- geoloc-convert: tool for converting csv to preprocessed database:
  `geoloc-convert in-csv-database-path out-preprocessed-database-path`
- geoloc-dummy: app with in-memory mockup database for benchmarking real app:
  `geoloc-dummy any-path`
- geoloc-check: tool for check some parameters of csv database:
  `geoloc-check csv-database-path`

## howto build

Tested with environment:
- linux
- ubuntu 22.04
- rust 1.63
- cargo
- csv crate for geoloc-convert & geoloc-check

Build steps:
- `cargo build -r`
- `cargo build -r -F build-csv`
- binaries located in `target/release` path

## databases schema

csv database:
- column[0]: uint32, ip from
- column[1]: uint32, ip to
- column[2]: ascii, country code
- column[5]: ascii, city

preprocessed database:
- uint64: LittleEndian, number of cities
- uint64: LittleEndian, number of ips
- array of cities: `| u8: length | [u8; 14]: ascii name of city |`
- array of ips: `| u32: ip addr | u1: end of range | u31: idx of my city | u32: idx of parent range city |`

## lookup algorithm

- sorted db by ip
- if ip is in db -> take my city idx -> take name of city
- ip is not in db -> take nearest ip -> take my city idx or parent city idx -> take name of city

## benchmarking

```
points = load_time_ms + memory_usage_mb * 10 + lookup_time_ms * 1000
```

### geoloc

```
Database loaded Memory usage: 70.13mb Load time: 46ms
OK    1.0.0.0 US Los Angeles Memory usage: 70.13mb Lookup time: 90μs
OK    71.6.28.0 US San Jose Memory usage: 70.13mb Lookup time: 61μs
OK    71.6.28.255 US San Jose Memory usage: 70.13mb Lookup time: 39μs
OK    71.6.29.0 US Concord Memory usage: 70.13mb Lookup time: 29μs
OK    53.103.144.0 DE Stuttgart Memory usage: 70.13mb Lookup time: 29μs
OK    53.255.255.255 DE Stuttgart Memory usage: 70.13mb Lookup time: 25μs
OK    54.0.0.0 US Rahway Memory usage: 70.13mb Lookup time: 52μs
OK    223.255.255.255 AU Brisbane Memory usage: 70.13mb Lookup time: 40μs
OK    5.44.16.0 GB Hastings Memory usage: 70.13mb Lookup time: 42μs
OK    8.24.99.0 US Hastings Memory usage: 70.13mb Lookup time: 41μs
Final points for 10 measurements:  793.284413
```

### geoloc-dummy

```
Database loaded Memory usage: 1.0mb Load time: 121μs
OK    1.0.0.0 US Los Angeles Memory usage: 1.0mb Lookup time: 378μs
OK    71.6.28.0 US San Jose Memory usage: 1.0mb Lookup time: 65μs
OK    71.6.28.255 US San Jose Memory usage: 1.0mb Lookup time: 41μs
OK    71.6.29.0 US Concord Memory usage: 1.0mb Lookup time: 23μs
OK    53.103.144.0 DE Stuttgart Memory usage: 1.0mb Lookup time: 17μs
OK    53.255.255.255 DE Stuttgart Memory usage: 1.0mb Lookup time: 16μs
OK    54.0.0.0 US Rahway Memory usage: 1.0mb Lookup time: 16μs
OK    223.255.255.255 AU Brisbane Memory usage: 1.0mb Lookup time: 16μs
OK    5.44.16.0 GB Hastings Memory usage: 1.0mb Lookup time: 16μs
OK    8.24.99.0 US Hastings Memory usage: 1.0mb Lookup time: 19μs
Final points for 10 measurements:  71.337874
```
