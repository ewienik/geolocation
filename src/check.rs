use {csv::Reader, std::path::PathBuf};

pub(crate) fn check(path: PathBuf) {
    println!(
        "number of elements: {}",
        Reader::from_path(&path).unwrap().records().count()
    );
    println!(
        "number of elements 'ip from' is xxx.xxx.xxx.0: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(0).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xff == 0)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.xxx.0.0: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(0).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffff == 0)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.0.0.0: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(0).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffffff == 0)
            .count()
    );
    println!(
        "number of elements 'ip from' is 0.0.0.0: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(0).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffffffff == 0)
            .count()
    );
    println!(
        "number of elements 'ip to' is xxx.xxx.xxx.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(1).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xff == 0xff)
            .count()
    );
    println!(
        "number of elements 'ip to' is xxx.xxx.255.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(1).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffff == 0xffff)
            .count()
    );
    println!(
        "number of elements 'ip to' is xxx.255.255.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(1).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffffff == 0xffffff)
            .count()
    );
    println!(
        "number of elements 'ip to' is 255.255.255.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(1).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xffffffff == 0xffffffff)
            .count()
    );
    println!(
        "number of elements 'ip from' is not to xxx.xxx.xxx.0: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(0).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xff != 0)
            .count()
    );
    println!(
        "number of elements 'ip to' is not to xxx.xxx.xxx.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| v.get(1).unwrap().parse().unwrap(),)
            .filter(|v: &u32| *v & 0xff != 0xff)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.xxx.0.0 when 'ip to' is yyy.yyy.255.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| (
                v.get(0).unwrap().parse().unwrap(),
                v.get(1).unwrap().parse().unwrap(),
            ))
            .filter(|(_, v2): &(u32, u32)| *v2 & 0xffff == 0xffff)
            .filter(|(v1, _)| *v1 & 0xffff == 0)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.0.0.0 when 'ip to' is yyy.255.255.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| (
                v.get(0).unwrap().parse().unwrap(),
                v.get(1).unwrap().parse().unwrap(),
            ))
            .filter(|(_, v2): &(u32, u32)| *v2 & 0xffffff == 0xffffff)
            .filter(|(v1, _)| *v1 & 0xffffff == 0)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.xxx.xxx.0 and 'ip to' is not xxx.xxx.xxx.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| (
                v.get(0).unwrap().parse().unwrap(),
                v.get(1).unwrap().parse().unwrap(),
            ))
            .filter(|(v1, v2): &(u32, u32)| *v1 & 0xffffff00 != *v2 & 0xffffff00)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.xxx.yyy.0 and 'ip to' is not xxx.xxx.zzz.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| (
                v.get(0).unwrap().parse().unwrap(),
                v.get(1).unwrap().parse().unwrap(),
            ))
            .filter(|(v1, v2): &(u32, u32)| *v1 & 0xffff0000 != *v2 & 0xffff0000)
            .count()
    );
    println!(
        "number of elements 'ip from' is xxx.yyy.yyy.0 and 'ip to' is not xxx.zzz.zzz.255: {}",
        Reader::from_path(&path)
            .unwrap()
            .records()
            .map(|v| v.unwrap())
            .map(|v| (
                v.get(0).unwrap().parse().unwrap(),
                v.get(1).unwrap().parse().unwrap(),
            ))
            .filter(|(v1, v2): &(u32, u32)| *v1 & 0xff000000 != *v2 & 0xff000000)
            .inspect(|(v1, v2)| println!("{v1:#08x} -> {v2:#08x}"))
            .count()
    );
}
