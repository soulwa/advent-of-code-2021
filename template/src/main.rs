use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| ())
        .collect::<Vec<_>>();

    Ok(())
}