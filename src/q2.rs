use std::fs;
use regex::Regex;

fn q2()
{
    let contents = fs::read_to_string("data/q2.txt")
        .expect("Something went wrong reading the file");
    let mut correct = 0;
    for line in contents.lines() {
        // 6-11 j: hcqcnjqjjgj
        let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
        let captures = re.captures(line).unwrap();
        // println!("{} {:?}", line, captures);
        let min: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let letter = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.get(4).unwrap().as_str();
        
        let mut count = 0;
        if password.chars().nth(min - 1).unwrap() == letter {
            count += 1;
        }
        if password.chars().nth(max - 1).unwrap() == letter {
            count += 1;
        }
        if count == 1 {
            correct += 1;
        }
    }

    println!("{}", correct);
}


fn main() {
    println!("Hello, world!");

    q2();
}
