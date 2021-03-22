use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0");

    let file = File::open("data/q14")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mem_reg = Regex::new(r##"mem\[(?P<mem>\d+)] = (?P<val>\d+)"##).unwrap();
    let mask_reg = Regex::new(r##"mask = (?P<mask>.*)"##).unwrap();

    let mut over = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = (0 as u64, 0 as u64);
    for line in lines.iter().map(|l| l.as_str()) {
        if line.starts_with("mem") {
            let caps = mem_reg.captures(line).unwrap();
            let mem = caps.name("mem").unwrap().as_str()
                .parse::<u64>().unwrap();
            let val = caps.name("val").unwrap().as_str().parse::<u64>().unwrap();
            let res = (val & mask.0) | mask.1;
            println!("{} => {}\n{:04$b}\n{:04$b}", line, res, val, res, 36);

            if let Some(v) = memory.insert(mem, res) { over += 1; }
        } else {
            let maskstr = mask_reg.captures(line).unwrap().name("mask").unwrap().as_str();
            mask.0 = 0;
            mask.1 = 0;
            for c in maskstr.chars() {
                mask.0 <<= 1;
                mask.1 <<= 1;
                match c {
                    '1' => {
                        mask.1 += 1;
                        mask.0 += 1;
                    }
                    '0' => (),
                    _ => { mask.0 += 1 }
                }
            }
            println!("mask\n{}\n{:03$b}\n{:03$b}", maskstr, mask.0, mask.1, 36);
        }
    }

    let mut total = 0 as u64;
    for v in memory.values() {
        total += *v;
    }
    dbg!(total);

    let mut keys = memory.keys().collect::<Vec<_>>();
    keys.sort();
    for k in keys {
        println!("{}: {}", k, memory[k]);
    }
    Ok(())
}

