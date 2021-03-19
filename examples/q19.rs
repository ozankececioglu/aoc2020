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
        "");

    let file = File::open("data/q16")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    enum Rule {
        Leaf,
        Sequence,
        Disjunction,
    }

    let rules: HashMap<i32, Rule> = HashMap::new();

    for line in lines {
        line.split(" | ");
    }

    Ok(())
}

