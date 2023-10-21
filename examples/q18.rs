use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor};

use std::{iter};

use regex::{Regex};




//
// fn part1() {
//     enum Op {
//         Nop,
//         Add,
//         Multiply,
//     }
//
//     impl Op {
//         fn perform(&self, arg1: i64, arg2: i64) -> i64 {
//             match self {
//                 Op::Add => arg1 + arg2,
//                 Op::Multiply => arg1 * arg2,
//                 Op::Nop => { arg2 }
//             }
//         }
//     }
//
//     // println!("{}", Op::Add.perform(3, 5));
//     // println!("{}", Op::Multiply.perform(3, 5));
//     // println!("{}", Op::Nop.perform(3, 5));
//
//     let re = Regex::new(r##"(\d+)|([)(+*])|(\s+)"##).unwrap();
//     let mut stack: Vec<(Op, i64)> = Vec::new();
//     let mut ax = 0 as i64;
//     let mut op = Op::Nop;
//     let mut total = 0 as i64;
//     for line in lines {
//         stack.clear();
//         ax = 0;
//         op = Op::Nop;
//         for find in re.find_iter(line.as_str()) {
//             let findstr = find.as_str();
//             match findstr.chars().next().unwrap() {
//                 '(' => {
//                     stack.push((op, ax));
//                     op = Op::Nop;
//                     ax = 0;
//                 }
//                 ')' => {
//                     let (pop, pax) = stack.pop().unwrap();
//                     ax = pop.perform(pax, ax);
//                     op = Op::Nop;
//                 }
//                 '+' => op = Op::Add,
//                 '*' => op = Op::Multiply,
//                 ' ' => {}
//                 _ => {
//                     let num = findstr.parse::<i64>().unwrap();
//                     ax = op.perform(ax, num);
//                     op = Op::Nop;
//                 }
//             }
//         }
//         println!("{}", ax);
//         total += ax as i64;
//     }
//     println!("{}", total);
// }


fn main() -> io::Result<()> {
    let _inp = Cursor::new(
        "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

    let file = File::open("data/q18")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    #[derive(Debug)]
    enum Token {
        Enter,
        Exit,
        Add,
        Multiply,
        Number(i64),
    }

    fn do_context<'a, T>(iter: &mut T) -> i64
        where T: Iterator<Item=Token>
    {
        let _result = 0 as i64;
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            /// Process Enter and Exit tokens
            let t = iter.next().unwrap_or(Token::Exit);
            match t {
                Token::Enter => tokens.push(Token::Number(do_context(iter))),
                Token::Exit => break,
                _ => tokens.push(t)
            }
        }

        let mut mults: Vec<i64> = Vec::new();
        let mut add_next= false;
        for t in tokens {
            match t {
                Token::Add => add_next = true,
                Token::Number(n) => {
                    if add_next {
                        *mults.last_mut().unwrap() += n;
                    } else {
                        mults.push(n);
                    }
                }
                _ => add_next = false
            }
        }
        return mults.iter().fold(1, |a, b| a * *b);
    }

    let re = Regex::new(r##"(\d+)|([)(+*])|(\s+)"##).unwrap();
    let mut total = 0 as i64;
    for line in lines {
        let mut iter = re.find_iter(line.as_str())
            .map(|m| m.as_str())
            .filter_map(|s| match s.chars().next().unwrap() {
                '(' => Some(Token::Enter),
                ')' => Some(Token::Exit),
                '+' => Some(Token::Add),
                '*' => Some(Token::Multiply),
                ' ' => None,
                _ => Some(Token::Number(s.parse::<i64>().unwrap()))
            }).chain(iter::once(Token::Exit));

        total += dbg!(do_context(&mut iter));
    }
    println!("{}", total);
    Ok(())
}