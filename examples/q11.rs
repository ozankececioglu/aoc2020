// #![feature(generators, generator_trait)]

use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor};








// use std::ops::Generator;
// use std::pin::Pin;


fn in_range<T: std::cmp::PartialOrd>(val: T, min: T, max: T) -> bool {
    return val > min && val < max;
}

fn print_seats(seats: &Vec<Vec<u8>>) {
    for i in seats {
        for j in i {
            print!("{}", match *j {
                0 => '.',
                1 => 'L',
                _ => '#'
            });
        }
        println!();
    }
}


fn main() -> io::Result<()> {
    let _inp = Cursor::new(
        "");

    let file = File::open("data/q11")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut seats1: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        seats1.push(line.chars().map(|c| if c == 'L' { 1 } else { 0 }).collect::<Vec<_>>());
    }
    let mut seats2 = seats1.clone();
    let mut turn = 0;

    dbg!("{}", turn);
    print_seats(&seats1);
    println!();

    let nb: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    let mut same_state = false;
    while !same_state {
        let curr: &mut Vec<Vec<u8>>;
        let next: &mut Vec<Vec<u8>>;
        if turn % 2 == 1 {
            curr = &mut seats1;
            next = &mut seats2;
        } else {
            curr = &mut seats2;
            next = &mut seats1;
        }
        turn += 1;

        same_state = true;
        for x in 0..curr.len() as i32 {
            'next_seat: for y in 0..curr[x as usize].len() as i32 {
                let seat = curr[x as usize][y as usize];
                if seat == 0 {
                    continue;
                }

                let mut occupied = 0;
                'next_dir: for n in nb.iter() {
                    let mut pos = (x, y);
                    loop {
                        pos.0 += n.0;
                        pos.1 += n.1;
                        let s = *curr.get(pos.0 as usize)
                            .map_or(
                                &(1 as u8),
                                |c: &Vec<u8>| c.get(pos.1 as usize)
                                    .unwrap_or(&(1 as u8)),
                            );
                        if s == 2 {
                            if seat == 2 {
                                occupied += 1;
                                if occupied >= 5 {
                                    next[x as usize][y as usize] = 1;
                                    same_state = false;
                                    continue 'next_seat;
                                } else {
                                    continue 'next_dir;
                                }
                            } else if seat == 1 {
                                next[x as usize][y as usize] = 1;
                                continue 'next_seat;
                            }
                        } else if s == 1 {
                            break;
                        }
                    }
                }

                next[x as usize][y as usize] = 2;
                if seat == 1 {
                    same_state = false;
                }
            }
        }

        println!();
        println!("=============== {}", turn);
        print_seats(&next);

        if same_state {
            println!("{} {}", next.iter().flat_map(|s| s).filter(|s| **s == 2).count(), turn);
        }
    }

    Ok(())
}

