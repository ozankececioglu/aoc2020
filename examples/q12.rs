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


enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

impl Direction {
    fn from(inp: &str) -> Direction {
        match inp.chars().nth(0).unwrap() {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            c => panic!("no such val {}", c)
        }
    }

    fn apply(&self, pos: &mut (i32, i32, i32), val: i32) {
        match self {
            Direction::North => pos.1 += val,
            Direction::South => pos.1 -= val,
            Direction::East => pos.0 += val,
            Direction::West => pos.0 -= val,
            Direction::Right => pos.2 -= val,
            Direction::Left => pos.2 += val,
            Direction::Forward => {
                match (pos.2 % 360 + 360) % 360 {
                    0 => pos.0 += val,
                    90 => pos.1 += val,
                    180 => pos.0 -= val,
                    270 => pos.1 -= val,
                    v => panic!("no such val {}", v)
                }
            }
        }
    }
}


fn main() -> io::Result<()> {
    let file = File::open("data/q12")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    
    let vals = lines.map(|l| {
        let captures = regex::Regex::new(r"(\w)(\d+)").unwrap().captures(&l).unwrap();
        (Direction::from(&captures[1]), captures[2].parse::<i32>())
    });

    // println!("{:?}", vals.collect::<Vec<_>>());

    let mut origin = (0, 0, 0);
    vals.for_each(|(d, v)| d.apply(&mut origin, v.unwrap()));

    println!("{:?}", origin);

    Ok(())
}