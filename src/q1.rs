use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data/q1.txt")
        .expect("Something went wrong reading the file");

    
    let mut numbers: HashSet<i32> = HashSet::new();
    for i in contents.lines() {
        numbers.insert(i.parse().unwrap());
    }

    for i in numbers.iter() {
        for j in numbers.iter() {
            if i != j {
                if numbers.contains(&(2020 - i - j)) {
                    println!("{}", i * j * (2020 - i - j));
                    return;
                }
            }
        }
        
    }
}
