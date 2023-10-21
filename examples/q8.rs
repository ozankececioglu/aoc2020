use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};


use std::collections::{HashSet};
use std::time::Instant;
use std::{fmt, fmt::{Display, Formatter}};


fn main() -> io::Result<()> {
    let _start = Instant::now();

    let _inp = Cursor::new(
        "nop +0
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

    impl Display for OpCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                OpCode::Acc(x) => write!(f, "Acc({})", x),
                OpCode::Nop(x) => write!(f, "Nop({})", x),
                OpCode::Jmp(x) => write!(f, "Jmp({})", x),
            };
            return Ok(());
        }
    }

    let mut ops: Vec<OpCode> = Vec::new();
    for line in lines {
        let mut it = line.split_whitespace();
        let op = it.next().unwrap();
        let val = it.next().unwrap().parse::<i32>().unwrap();
        ops.push(match op {
            "acc" => Ok(OpCode::Acc(val)),
            "nop" => Ok(OpCode::Nop(val)),
            "jmp" => Ok(OpCode::Jmp(val)),
            _ => Err("qwe")
        }.unwrap());
    }

    struct OpState<'a> {
        name: &'a str,
        ip: usize,
        visits: HashSet<usize>,
        acc: i32,
        ops: &'a Vec<OpCode>,
    }

    impl<'a> OpState<'a> {
        fn advance(&mut self) {
            self._step(false);
        }

        fn fork(&mut self) {
            self._step(true);
        }

        fn _step(&mut self, rev: bool) {
            let previp = self.ip;
            self.visits.insert(self.ip);
            match self.ops[self.ip] {
                OpCode::Acc(x) => {
                    self.acc += x;
                    self.ip += 1;
                }
                OpCode::Jmp(x) => self.ip = if !rev { (self.ip as i32 + x) as usize } else { self.ip + 1 },
                OpCode::Nop(x) => self.ip = if rev { (self.ip as i32 + x) as usize } else { self.ip + 1 },
            }
            println!("{}: {} -> {} -> {}", self.name, previp, self.ops[previp], self.ip);
        }

        fn is_acc(&self) -> bool {
            match self.ops[self.ip] {
                OpCode::Acc(_) => true,
                _ => false
            }
        }

        fn get_op(&self) -> &OpCode { &self.ops[self.ip] }

        fn new(ops_arg: &'a Vec<OpCode>, name: &'a str) -> OpState<'a> {
            return OpState {
                name,
                ip: 0,
                visits: HashSet::new(),
                acc: 0,
                ops: ops_arg,
            };
        }
    }

    let mut state: OpState = OpState::new(&ops, "orig");
    let mut fstate: OpState = OpState::new(&ops, "fork");

    loop {
        while state.is_acc() {
            state.advance();
        }

        fstate.visits.clear();
        fstate.ip = state.ip;
        fstate.acc = state.acc;
        fstate.fork();

        while !state.visits.contains(&fstate.ip) & &!fstate.visits.contains(&fstate.ip) {
            if fstate.ip < 0 || fstate.ip > ops.len() {
                break;
            }
            if fstate.ip == ops.len() {
                println!("!### {}", fstate.acc);
                return Ok(());
            }
            fstate.visits.insert(fstate.ip);
            fstate.advance();
        }

        if state.visits.contains(&state.ip) {
            // println!("failed {}", );
            return Err(Error::from(ErrorKind::InvalidData));
        }
        state.advance();
    }

    // println!("{} {} {}", acc, visits.len(), ops.len());


    Ok(())
}