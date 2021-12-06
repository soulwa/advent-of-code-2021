use std::collections::HashMap;
use std::io::{Read, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    BufReader::new(File::open("input")?)
        .read_to_string(&mut input)?;

    let input = input.split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    // part 1 = 80, part 2 = 256
    let num_days = 256;

    let res: i64 = input.iter().map(|l| calc_num_fish(&mut cache, *l, num_days)).sum();

    println!("{}", res);

    Ok(())
}

fn calc_num_fish(cache: &mut HashMap<(i64, i64), i64>, ft: i64, dr: i64) -> i64 {
    match cache.get(&(ft, dr)) {
        Some(val) => *val,
        None => {
            let result = if ft > dr || dr == 0 { 
                1
            } else if ft == 0 {
                calc_num_fish(cache, 6, dr - 1) + calc_num_fish(cache, 8, dr - 1)
            } else {
                calc_num_fish(cache, ft - 1, dr - 1)
            };
            cache.insert((ft, dr), result);
            result
        }
    }
}