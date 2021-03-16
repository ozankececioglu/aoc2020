use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::iter::Map;


fn main() -> io::Result<()> {
    let file = File::open("data/q3")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    println!("{} {}", lines.len(), lines[0].len());


    let mut trees = 0;
    let inc = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut res: i64 = 1;
    for (xinc, yinc) in inc.iter() {
        let mut x = 0;
        let mut y = 0;
        while y < lines.len() {
            if lines[y].as_bytes()[x] == '#' as u8 {
                trees += 1;
            }

            y += yinc;
            x = (x + xinc) % lines[0].len();
        }
        println!("{} {} {}", xinc, yinc, trees);
        res *= trees;
        trees = 0;
    }

    println!("{}", res);

    Ok(())
}