use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use num::integer::lcm;
use num::bigint::BigInt;
use itertools::Itertools;


fn chinese_remainder_theorem(vals: &Vec<(BigInt, BigInt)>) -> BigInt {
    
    let prod: BigInt = vals.iter().map(|(_, m)| m).product();
    println!("prod: {}", prod);
    
    let mut sum = BigInt::from(0u32);
    
    for (residue, modulus) in vals.iter() {
        let p = &prod / modulus;
        sum += residue * mod_inv(&p, modulus) * p;
    }

    sum % prod
}

/// x ^ (m - 2) % m
/// 
fn mod_inv(x: &BigInt, modulus: &BigInt) -> BigInt {
    let a = modulus - BigInt::from(2u32);
    x.modpow(&a, modulus)
}



fn main() -> io::Result<()> {
    let file = File::open("data/q13")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    lines.next();

    let buses = lines.next().unwrap().split(',')
        .map(|b| b.parse::<BigInt>().unwrap_or(BigInt::from(0)))
        .enumerate()
        .filter(|x| x.1 > BigInt::from(0))
        .map(|(x, y)| (&y - x, y))
        .collect::<Vec<_>>();    

    // 7,13,x,x,59,x,31,19
    // let buses = vec![
    //     (0, BigInt::from(7)), 
    //     (12, BigInt::from(13)), 
    //     (55, BigInt::from(59)),
    //     (25, BigInt::from(31)),
    //     (12, BigInt::from(19))
    // ];

    let res = chinese_remainder_theorem(&buses);

    println!("{:?}", res);

    let res = 1068781;

    for (i, b) in buses.iter() {
        println!("{}: {}, {}", i, b, &res % b);
    }
    
    Ok(())
}

