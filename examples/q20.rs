use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use itertools::{sorted};


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "");

    let file = File::open("data/q20")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let reg = Regex::new(r##"Tile (?P<tile>\d+):$"##).unwrap();

    fn left_border(image: &Vec<String>) -> String {
        let mut res = String::new();
        for r in image {
            res.push(r.chars().next().unwrap());
        }
        return res;
    }
    fn right_border(image: &Vec<String>) -> String {
        let mut res = String::new();
        for r in image {
            res.push(r.chars().last().unwrap());
        }
        return res;
    }


    let mut borders: HashMap<String, Vec<u64>> = HashMap::new();
    loop {
        let num = reg.captures(lines.next().unwrap().as_str()).unwrap()
            .name("tile").unwrap().as_str()
            .parse::<u64>().unwrap();
        let mut image = Vec::new();
        for i in 0..10 {

            let row = lines.next().unwrap();
            image.push(row);
        }

        let bs = [image[0].clone(), image.last().unwrap().clone(), left_border(&image), right_border(&image)].to_vec();
        for b in bs.into_iter() {
            borders.entry(b.clone()).or_insert(Vec::new()).push(num);
            borders.entry(b.chars().rev().collect()).or_insert(Vec::new()).push(num);
        }

        if lines.next().map_or(true, |_| false) {
            break;
        }
    }

    let mut bcounts = HashMap::new();
    for b in borders {
        if b.1.len() == 1 {
            *bcounts.entry(b.1[0]).or_insert(0) += 1;
        }
    }

    // let res = bcounts.iter().filter(|v| *v.1 == 2).fold(1, |i, v| i * *v.1);
    // dbg!(res);
    let mut a = bcounts.iter().collect::<Vec<_>>();
    a.sort_by(|a, b| a.0.cmp(b.0));
    dbg!(a);

    let res = bcounts.iter().filter(|v| *v.1 == 4).fold(1, |i, v| i * *v.0);
    dbg!(res);

    // dbg!(res);

    // dbg!(images.len());
    Ok(())
}

