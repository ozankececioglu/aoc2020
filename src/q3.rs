use std::fs;
use std::vec;
use std::slice::Iter;
use std::iter::Map;

fn main() {
    let contents = fs::read_to_string("data/q1.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
    let mut x: usize = 0;
    let mut y: usize = 0;
    
    let mut trees = 0;
    while y < lines.len() {
        for line in lines {
            for c in line {
                if c == '#' {
                    trees += 1;
                }
            }
        }

        x += 3;
        y += 1;

        x = x % lines[0].len();
    }
    println!("{}", trees);
}