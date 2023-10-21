use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor};


use std::collections::{VecDeque};


use linked_hash_set::LinkedHashSet;


fn main() -> io::Result<()> {
    let _inp = Cursor::new(
        "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576");

    let file = File::open("data/q9")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut all: Vec<i64> = Vec::new();
    let mut numbers: LinkedHashSet<i64> = LinkedHashSet::new();
    while numbers.len() < 25 {
        numbers.insert(lines.next().unwrap().parse().unwrap());
    }
    let mut found: i64 = 0;
    'outer: for line in lines {
        let number = line.parse::<i64>().unwrap();

        for _i in numbers.iter() {
            if !numbers.iter().any(|n| numbers.contains(&(number - n))) {
                found = number;
                break 'outer;
            }
        }

        all.push(number);
        numbers.pop_front();
        numbers.insert(number);
    };

    numbers.clear();
    println!("{}", found);

    let mut que: VecDeque<i64> = VecDeque::new();
    let mut total: i64 = 0;
    for number in all.iter() {
        if total < found {
            total += number;
            que.push_front(*number);
        }
        if total == found {
            break;
        }
        while total > found {
            total -= que.pop_back().unwrap();
        }
    }

    println!("!### {:?} {}", que, que.iter().min().unwrap() + que.iter().max().unwrap());


    Ok(())
}