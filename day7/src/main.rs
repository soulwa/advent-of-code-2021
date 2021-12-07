use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .next().unwrap();

    let min: i32 = input.iter().map(|i| input.iter().map(move |elem| {
        if elem == i {
            0
        } else {
            (0..i32::abs(elem - i)+1).sum::<i32>()
        }
    }).sum::<i32>()).min().unwrap();

    println!("minimum alignment: {:?}", min);

    Ok(())
}