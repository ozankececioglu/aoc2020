use std::{fs::File};
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

impl Direction {
    fn from(inp: &str) -> Direction {
        match inp.chars().nth(0).unwrap() {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            c => panic!("no such val {}", c)
        }
    }

    fn apply(&self, pos: &mut (i32, i32), wp: &mut (i32, i32), val: &i32) {
        match self {
            Direction::North => wp.1 += val,
            Direction::South => wp.1 -= val,
            Direction::East => wp.0 += val,
            Direction::West => wp.0 -= val,
            Direction::Right | Direction::Left => {
                let mut angle = ((val % 360) + 360 % 360) / 90;
                if *self == Direction::Right {
                    angle = 4 - angle;
                }
                for _ in 0..angle {
                    let tmp = wp.0;
                    wp.0 = -wp.1;
                    wp.1 = tmp;
                }
            },
            Direction::Forward => {
                pos.0 += wp.0 * val;
                pos.1 += wp.1 * val;
            }
        }
    }
}


fn main() -> io::Result<()> {
    let file = File::open("data/q12")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    
    let reg = regex::Regex::new(r"(\w)(\d+)").unwrap();
    let vals = lines.map(|l| {
        let captures = reg.captures(&l).unwrap();
        (Direction::from(&captures[1]), captures[2].parse::<i32>().unwrap())
    });

    let mut origin = (0, 0);
    let mut waypoint = (10, 1);
    vals.for_each(|(d, v)| d.apply(&mut origin, &mut waypoint, &v));

    println!("{:?}", origin);

    Ok(())
}