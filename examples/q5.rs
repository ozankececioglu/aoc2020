use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::iter::Map;
use std::collections::btree_set::BTreeSet;


fn parse_seat(t: &str) -> i32 {
    let mut row = 0;
    let mut col = 0;
    for (i, c) in t[0..7].chars().enumerate() {
        if c == 'B' { row += 1 << (6 - i) }
    }
    for (i, c) in t[7..].chars().enumerate() {
        if c == 'R' { col += 1 << (2 - i) }
    }
    // println!("{} {}", row, col);
    return row * 8 + col;
}


fn main() -> io::Result<()> {
    let file = File::open("data/q5")?;
    let reader = BufReader::new(file);
    let mut max_ticket = 0;
    let mut seats = Vec::new();


    for line in reader.lines().map(|l|l.unwrap()) {
        println!("{}", line);
        let t = parse_seat(line.as_str());
        // if t > max_ticket { max_ticket = t }
        seats.push(t);
    }

    seats.sort();
    println!("{:?}", seats);

    for i in 0..seats.len() {
        if seats[i] + 1 != seats[i + 1] {
            println!("{}", seats[i] + 1);
            break;
        }
    }

    // println!("{}", max_ticket);
    Ok(())
}
