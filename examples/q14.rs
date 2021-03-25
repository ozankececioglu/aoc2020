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
        "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1");

    let file = File::open("data/q14")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mem_reg = Regex::new(r##"mem\[(?P<mem>\d+)] = (?P<val>\d+)"##).unwrap();
    let mask_reg = Regex::new(r##"mask = (?P<mask>.*)"##).unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for line in lines.iter().map(|l| l.as_str()) {
        if line.starts_with("mem") {
            let caps = mem_reg.captures(line).unwrap();
            let mut address = caps.name("mem").unwrap().as_str()
                .parse::<u64>().unwrap();
            let val = caps.name("val").unwrap().as_str().parse::<u64>().unwrap();

            // println!("{}\n{:02$b}", mask, address, 36);
            let mut comb: Vec<usize> = Vec::new();
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '1' => address = address | (1 << i),
                    'X' => comb.push(i),
                    _ => ()
                }
            }

            // println!("{:01$b}", address, 36);

            fn make_comb(memory: &mut HashMap<u64, u64>, address: u64, val: u64, comb: &Vec<usize>, i: usize) {
                if i < comb.len() {
                    let j = comb[i];
                    make_comb(memory, address | (1 << j), val, comb, i + 1);
                    make_comb(memory, address & !(1 << j), val, comb, i + 1);
                } else {
                    memory.insert(address, val);
                    // println!("{:01$b}", address, 36);
                }
            }
            make_comb(&mut memory, address, val, &comb, 0);
            // println!("-----");
        } else {
            mask = mask_reg.captures(line).unwrap().name("mask").unwrap().as_str();
        }
    }

    let mut total = 0 as u64;
    for v in memory.values() {
        total += *v;
    }
    dbg!(total);
    //
    // let mut keys = memory.keys().collect::<Vec<_>>();
    // keys.sort();
    // for k in keys {
    //     println!("{}: {}", k, memory[k]);
    // }
    Ok(())
}

