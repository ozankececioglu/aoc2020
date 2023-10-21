use std::{fs::File};

use std::io::{self, prelude::*, BufReader, Cursor};







fn main() -> io::Result<()> {
    let _inp = Cursor::new(
        "");

    let file = File::open("data/q16")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    for _line in lines {}

    Ok(())
}

