use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use std::hash::Hash;
use linked_hash_set::LinkedHashSet;


fn main() -> io::Result<()> {
    let inp = Cursor::new("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6");

    let file = File::open("data/q8")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    enum OpCode {
        Nop(i32),
        Acc(i32),
        Jmp(i32),
    }

    let mut ops: Vec<OpCode> = Vec::new();
    for line in lines {
        let mut it = line.split_whitespace();
        ops.push(match it.next().unwrap() {
            "acc" => Ok(OpCode::Acc(it.next().unwrap().parse::<i32>().unwrap())),
            "nop" => Ok(OpCode::Nop(it.next().unwrap().parse::<i32>().unwrap())),
            "jmp" => Ok(OpCode::Jmp(it.next().unwrap().parse::<i32>().unwrap())),
            _ => Err("qwe")
        }.unwrap());
    }

    #[derive(Clone, Default)]
    struct OpState {
        i: usize,
        visit_set: HashSet<usize>,
        visit_order: Vec<usize>,
        acc: i32,
    }

    fn part1(prev_visit: &HashSet<usize>, mut acc: i32, mut ip: usize) {
        let mut visited: HashSet<usize> = HashSet::new();
        // let mut acc = 0;
        // let mut ip: usize = 0; // instruction pointer

        while !visited.contains(&ip) {
            visited.insert(ip);
            // println!("{}", i);
            match ops[ip as usize] {
                OpCode::Acc(x) => {
                    acc += x;
                    ip += 1;
                }
                OpCode::Jmp(x) => ip = (ip as i32 + x) as usize,
                OpCode::Nop(_) => ip += 1
            }
        }
        println!("{} {} {}", acc, visited.len(), ops.len());
    }

    // Part 1


    // Part 2
    let mut forked_at: Option<usize> = None;

    Ok(())
}