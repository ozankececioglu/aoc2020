use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;


fn main() -> io::Result<()> {
    let file = File::open("data/q8")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    for line in lines {

    }

    Ok(())
}