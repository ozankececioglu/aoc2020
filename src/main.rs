mod util;

use std::collections::HashMap;

fn main() {
    let a = ["qwe", "rt"];
    for i in a.iter().chain(["qwe"].iter()) {
        println!("{}", i);
    }
}
