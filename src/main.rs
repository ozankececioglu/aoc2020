mod util;

use std::collections::HashMap;

enum AEnum {
    AVal(i32),
    BVal,
    CVal(String),
}


fn test(i: &mut i32, k: i32) {
    *i += 1;
}

fn test2() {
    let op = AEnum::AVal(3);
    loop {
        match op {
            AEnum::AVal(_) => {}
            AEnum::BVal => {
                break;
            }
            AEnum::CVal(_) => {}
        }
    }
}

fn main() {
    let a = 3;
    let b = 5;
    let c: Vec<i32> = (0..5).collect();
    println!("{:?}", c);
}
