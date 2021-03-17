use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::HashSet;
use regex::{Regex, Captures};


struct Validator {
    height: Regex,
    hair_color: Regex,
    eye_color: HashSet<&'static str>,
}

impl Validator {
    fn new() -> Validator {
        return Validator {
            height: Regex::new(r#"(\d+)(cm|in)"#).unwrap(),
            hair_color: Regex::new(r#"#[0-9a-f]{6}"#).unwrap(),
            eye_color: ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect(),
        };
    }

    fn is_valid(&self, field: &str, value: &str) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        //  If cm, the number must be at least 150 and at most 193.
        //  If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.
        match field {
            "byr" => {
                let y = value.parse::<i32>().unwrap();
                y >= 1920 && y <= 2002
            }
            "iyr" => {
                let y = value.parse::<i32>().unwrap();
                y >= 2010 && y <= 2020
            }
            "eyr" => {
                let y = value.parse::<i32>().unwrap();
                y >= 2020 && y <= 2030
            }
            "hgt" => {
                self.height.captures(value).map_or(false, |caps| {
                    let h = caps.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
                    let u = caps.get(2).map_or("cm", |m| m.as_str());
                    if u == "cm" { h >= 150 && h <= 193 } else { h >= 59 && h <= 76 }
                })
            }
            "hcl" => {
                self.hair_color.is_match(value)
            }
            "ecl" => {
                self.eye_color.contains(value)
            }
            "pid" => {
                value.len() == 9 && value.parse::<i32>().is_ok()
            }
            "cid" => {
                true
            }
            _ => { false }
        }
    }
}


fn main() -> io::Result<()> {
    let file = File::open("data/q4")?;
    let reader = BufReader::new(file);

    let required: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter().cloned().collect();
    println!("!### {}", required.len());

    let mut nvalid = 0;
    let mut ninvalid = 0;
    let mut all = required.clone();
    let mut end = false;
    let validator = Validator::new();

    for line in reader.lines().map(|l| l.unwrap()).chain(iter::once(String::from(""))) {
        if line.is_empty() {
            all.remove("cid");
            if all.len() > 0 {
                ninvalid += 1;
            } else {
                nvalid += 1;
            }

            all = required.clone();
            end = false;
        } else if !end {
            for field in line.split_whitespace() {
                let x: Vec<_> = field.split(':').collect();

                if !validator.is_valid(x[0], x[1]) {
                    println!("{}", field);
                    end = true;
                    break;
                }

                if !all.remove(x[0]) {
                    println!("!###");
                }
            }
        }
    }
    println!("{} {}", nvalid, ninvalid);

    Ok(())
}