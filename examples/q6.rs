use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};


fn main() -> io::Result<()> {
    let file = File::open("data/q6")?;
    let reader = BufReader::new(file);

    let mut questions: HashMap<char, i32> = HashMap::new();
    let mut group = 0;
    let mut total = 0;

    for line in reader.lines().map(|l| l.unwrap())
        .chain(iter::once(String::from(""))) {
        if line.is_empty() {
            // println!("{:?}", questions);
            total += questions.iter().filter(|(_, v)| **v == group).count();
            questions.clear();
            group = 0;
        } else {
            for c in line.chars() {
                let count = questions.entry(c).or_insert(0);
                *count += 1;
            }
            // println!("{} {}", line, group);
            group += 1;
        }
    }

    println!("{}", total);

    Ok(())
}