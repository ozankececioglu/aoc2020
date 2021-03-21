use std::{fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};


use std::collections::{HashMap};
use regex::{Regex};



fn main() -> io::Result<()> {
    let file = File::open("data/q7")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let mut rels: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    let mut inv_rels: HashMap<&str, Vec<&str>> = HashMap::new();

    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    // dotted black bags contain no other bags.

    let reg = Regex::new(r##"(\d+|no) (.*?) bags?"##).unwrap();
    for line in lines.iter() {
        let mut it = line.split(" bags contain ");
        let noun = it.next().unwrap();
        let rest = it.next().unwrap();
        for i in rest.split(", ") {
            let m = reg.captures(i).unwrap();
            let count = m.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0);
            if count > 0 {
                let color = m.get(2).unwrap().as_str();
                rels.entry(noun).or_insert(Vec::new()).push((color, count));
                inv_rels.entry(color).or_insert(Vec::new()).push(noun);
            }
        }
    }

    println!("{:#?}", inv_rels);

    let mut targets: Vec<(&str, i32)> = vec![("shiny gold", 1)];
    let mut total = 0;
    while !targets.is_empty() {
        let (t, count) = targets.pop().unwrap();
        total += count;
        for (child, i) in rels.get(t).unwrap_or(&Vec::new()).iter() {
            targets.push((child, i * count));
        }
    }
    println!("{}", total - 1);

    /// Part 1
    // let mut targets: Vec<(&str, Vec<&str>)> = vec! {("shiny gold", Vec::new())};
    // let mut visited: HashSet<&str> = HashSet::new();
    // while !targets.is_empty() {
    //     let (t, history) = targets.pop().unwrap();
    //     for c in inv_rels.get(t).unwrap_or(&Vec::new()).iter() {
    //         if !visited.contains(c) {
    //             let mut h = history.clone();
    //             h.push(t);
    //             println!("{:?} + {}", h, *c);
    //             targets.push((*c, h));
    //         }
    //     }
    //     visited.insert(t);
    // }

    // println!("{}", visited.len());
    // println!("{:?}", visited);

    Ok(())
}