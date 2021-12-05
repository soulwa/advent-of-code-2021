use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let mut last = input[0];
    let mut count = 0;
    for item in input[1..].iter() {
        if *item > last {
            count += 1;
        }
        last = *item;
    }

    println!("{}", count);

    // part 2
    last = input[0] + input[1] + input[2];
    count = 0;
    for window in input[1..].windows(3) {
        if window.iter().sum::<i32>() > last {
            count += 1;
        }
        last = window.iter().sum()
    }

    println!("{}", count);

    Ok(())
}