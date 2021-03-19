use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::{Index};


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "3,1,2");

    // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.

    let str = std::fs::read_to_string("data/q15").unwrap();
    let mut lines = str.split(",")
        .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut spoken: HashMap<i64, i64> = HashMap::new();
    let mut turn = 1;
    let mut lastnum = lines[0];
    for i in lines[1..].iter() {
        spoken.insert(lastnum, turn);
        lastnum = *i;
        turn += 1;
        // dbg!((turn, lastnum));
    }

    while turn < 30000000 {
        lastnum = match spoken.insert(lastnum, turn) {
            Some(old) => turn - old,
            None => 0
        };
        turn += 1;
        // dbg!((turn, lastnum));
    }
    dbg!(turn, lastnum);

    Ok(())
}

