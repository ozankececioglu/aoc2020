use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor};


use std::collections::{HashSet};
use regex::{Regex};




fn main() -> io::Result<()> {
    let _inp = Cursor::new(
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");

    let file = File::open("data/q16")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    #[derive(Debug)]
    struct TicketField {
        name: String,
        range1: (i32, i32),
        range2: (i32, i32),
        possible: HashSet<usize>,
    }

    impl TicketField {
        fn check(&self, val: i32) -> bool {
            return val >= self.range1.0 && val <= self.range1.1 ||
                val >= self.range2.0 && val <= self.range2.1;
        }
        fn process(&mut self, val: i32, rank: usize) {
            let res =  val >= self.range1.0 && val <= self.range1.1 ||
                val >= self.range2.0 && val <= self.range2.1;
            if !res {
                self.possible.remove(&rank);
            }
        }
    }

    let re = Regex::new(r##"(.+?): (\d+)-(\d+) or (\d+)-(\d+)"##).unwrap();
    let mut fields: Vec<TicketField> = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let cap = re.captures(&line).unwrap();
        fields.push(TicketField {
            name: cap.get(1).unwrap().as_str().to_string(),
            range1: (cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                     cap.get(3).unwrap().as_str().parse::<i32>().unwrap()),
            range2: (cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                     cap.get(5).unwrap().as_str().parse::<i32>().unwrap()),
            possible: HashSet::new(),
        });
    }

    let len = fields.len();
    for f in fields.iter_mut() {
        f.possible = (0..len).collect();
    }

    // your ticket
    lines.next();
    // your ticket #
    let your_tickets: Vec<i32> = lines.next().unwrap().split(",")
        .map(|k| k.parse::<i32>().unwrap())
        .collect();
    // println!("{}", your_tickets);
    // blank
    lines.next();
    // nearby tickets
    lines.next();

    // let mut error_rate = 0;
    let mut nums: Vec<i32> = Vec::new();
    'outer: for line in lines {
        nums.clear();
        for val in line.split(',') {
            let parsed = val.parse::<i32>().unwrap();
            nums.push(parsed);
            if !fields.iter().any(|f| f.check(parsed)) {
                continue 'outer;
            }
        }
        for (i, val) in nums.iter().enumerate() {
            fields.iter_mut().for_each(|f| f.process(*val, i));
        }
    }

    fields.sort_by(|a, b| a.possible.len().cmp(&b.possible.len()));
    for i in 0..fields.len() {
        let n = *fields[i].possible.iter().next().unwrap();
        for j in (i+1)..fields.len() {
            fields[j].possible.remove(&n);
        }
    }


    let iters = fields.iter().filter(|f| f.name.contains("departure"))
        .fold(1 as i64, |a, b| a * your_tickets[*b.possible.iter().next().unwrap()] as i64);
    println!("{:#?}", iters);


    // println!("{}", error_rate);

    Ok(())
}




























