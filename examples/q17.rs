use std::{fs, fs::File};
use std::vec;
use std::cell;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use std::char::UNICODE_VERSION;
use std::cell::Cell;


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        ".#.
..#
###
");
    // 112

    let file = File::open("data/q17")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    #[derive(Hash, Eq, PartialEq)]
    struct Pos {
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    }

    impl Pos {
        fn add(&self, other: &Pos) -> Pos {
            return Pos {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            };
        }
    }

    // let neighbors = [
    //     Pos { x: 0, y: 0, z: 1 },
    //     Pos { x: 0, y: 0, z: -1 },
    //     Pos { x: 0, y: 1, z: 1 },
    //     Pos { x: 0, y: 1, z: -1 },
    //     Pos { x: 0, y: 1, z: 0 },
    //     Pos { x: 0, y: -1, z: 1 },
    //     Pos { x: 0, y: -1, z: -1 },
    //     Pos { x: 0, y: -1, z: 0 },
    //
    //     Pos { x: 1, y: 0, z: 0 },
    //     Pos { x: 1, y: 0, z: 1 },
    //     Pos { x: 1, y: 0, z: -1 },
    //     Pos { x: 1, y: 1, z: 1 },
    //     Pos { x: 1, y: 1, z: -1 },
    //     Pos { x: 1, y: 1, z: 0 },
    //     Pos { x: 1, y: -1, z: 1 },
    //     Pos { x: 1, y: -1, z: -1 },
    //     Pos { x: 1, y: -1, z: 0 },
    //
    //     Pos { x: -1, y: 0, z: 0 },
    //     Pos { x: -1, y: 0, z: 1 },
    //     Pos { x: -1, y: 0, z: -1 },
    //     Pos { x: -1, y: 1, z: 1 },
    //     Pos { x: -1, y: 1, z: -1 },
    //     Pos { x: -1, y: 1, z: 0 },
    //     Pos { x: -1, y: -1, z: 1 },
    //     Pos { x: -1, y: -1, z: -1 },
    //     Pos { x: -1, y: -1, z: 0 },
    // ];

    let mut neighbors: Vec<Pos> = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    neighbors.push(Pos{
                        x, y, z, w
                    });
                }
            }
        }
    }
    neighbors.remove(neighbors.len() / 2);

    let mut active: HashMap<Pos, Cell<i32>> = HashMap::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert(Pos { x: x as i32, y: y as i32, z: 0, w: 0 }, Cell::new(0));
            }
        }
    }

    for r in 0..6 {
        let mut passive: HashMap<Pos, i32> = HashMap::new();
        for (k, v) in active.iter() {
            v.set(0);
            for n in neighbors.iter() {
                let p = k.add(n);
                if active.contains_key(&p) {
                    v.set(v.get() + 1);
                }
                *passive.entry(p).or_insert(0) += 1;
            }
        }

        active.retain(|_, v| v.get() == 2 || v.get() == 3);
        for (k, v) in passive.into_iter() {
            if v == 3 {
                active.insert(k, Cell::new(0));
            }
        }

        println!("{}", active.len());
    }

    Ok(())
}