use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::iter::Map;


fn main() -> io::Result<()> {
    let file = File::open("data/q4")?;
    let reader = BufReader::new(file);



    Ok(())
}