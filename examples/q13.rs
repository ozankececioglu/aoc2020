use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use num::integer::lcm;
use num::bigint::BigInt;
use itertools::Itertools;


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "");

    let file = File::open("data/q13")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut depart = lines.next().unwrap().parse::<u32>().unwrap();
    let buses = lines.next().unwrap().split(',')
        .map(|b| b.parse::<u32>().unwrap_or(0))
        .enumerate()
        .filter(|(i, b)| *b > 0)
        .collect::<Vec<_>>();

    let mut it = buses.into_iter();
    let (i, m) = it.next().unwrap();
    let mut result = BigInt::from(0);
    for next in it.into_iter() {
        let k = (next.0 - i) as i32; /// diff


        let n = BigInt::from(next.1);
        //
        let (i, m) = next;
    }



    // dbg!(mintime * bus);

    Ok(())
}

