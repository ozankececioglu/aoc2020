use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "16
10
15
5
1
11
7
19
6
12
4");

    let file = File::open("data/q10")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let mut nums = lines.iter().map(|x| x.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nums.push(0);
    nums.sort();

    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(nums.len() - 1, 1);

    fn permutate(step: usize, nums: &Vec<usize>, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(x) = cache.get(&step) {
            return *x;
        }

        let mut total = 0;
        for i in 1..=3 {
            if step + i < nums.len() && nums[step + i] - nums[step] <= 3 {
                total += permutate(step + i, nums, cache);
            }
        }
        cache.insert(step, total);
        return total;
    }

    let res = permutate(0, &nums, &mut cache);

    let mut vcache = cache.iter().map(|x| (x.0, nums[*x.0], x.1)).collect::<Vec<_>>();
    vcache.sort();
    dbg!(nums);
    print!("{:?}", vcache);
    dbg!(res);
    Ok(())
}

