use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use arrayvec::ArrayVec;


fn in_range<T: std::cmp::PartialOrd>(val: T, min: T, max: T) -> bool {
    return val > min && val < max;
}

fn print_seats(seats: &Vec<Vec<u8>>) {
    for i in seats {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "");

    let file = File::open("data/q11")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut seats1: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        seats1.push(line.chars().map(|c| if c == 'L' { 1 } else { 0 }).collect::<Vec<_>>());
    }
    let mut seats2 = seats1.clone();
    let mut turn = 0;

    let nb: [(i32, i32); 8] = [(-1, -1), (0, -1), (-1, 0), (0, 1), (1, 0), (1, 1), (1, -1), (-1, 1)];

    let mut same_state = false;
    while !same_state {
        let curr: &mut Vec<Vec<u8>>;
        let next: &mut Vec<Vec<u8>>;
        if turn % 2 == 1 {
            curr = &mut seats1;
            next = &mut seats2;
        } else {
            curr = &mut seats2;
            next = &mut seats1;
        }
        turn += 1;

        same_state = true;
        for x in 0..curr.len() as i32 {
            'next_seat: for y in 0..curr[x as usize].len() as i32 {
                if curr[x as usize][y as usize] == 2 {
                    let mut occupied = 0;
                    for n in nb.iter() {
                        if curr.get((n.0 + x) as usize)
                            .map_or(
                                false,
                                |c: &Vec<u8>| c.get((n.1 + y) as usize)
                                    .map_or(false, |d| *d == (2 as u8)),
                            ) {
                            occupied += 1;
                            if occupied >= 4 {
                                next[x as usize][y as usize] = 1;
                                same_state = false;
                                continue 'next_seat;
                            }
                        }
                    }
                    next[x as usize][y as usize] = 2;

                } else if curr[x as usize][y as usize] == 1 {
                    for n in nb.iter() {
                        if curr.get((n.0 + x) as usize)
                            .map_or(
                                false,
                                |c: &Vec<u8>| c.get((n.1 + y) as usize)
                                    .map_or(false, |d| *d == (2 as u8)),
                            ) {
                            next[x as usize][y as usize] = 1;
                            continue 'next_seat;
                        }
                    }

                    next[x as usize][y as usize] = 2;
                    same_state = false;
                }
            }
        }

        println!("{}", turn);
        print_seats(&curr);

        if same_state {
            println!("{} {}", next.iter().flat_map(|s| s).filter(|s| **s == 2).count(), turn);
        }
    }

    Ok(())
}

