use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| u32::from_str_radix(&l.unwrap(), 2).unwrap())
        .collect::<Vec<_>>();

    // 5 for test, 12 for real
    const NUML: usize = 12;
    let mask: u32 = {
        let mut base: u32 = 0;
        for i in 0..NUML {
            base |= 1 << i
        }
        base
    };

    let mut most_common = [0; NUML];

    for i in 0..NUML {
        most_common[i] = if most_common_bit(&input, i as u32) { 1 } else { 0 }
    }

    let res: u32 = u32_from_n_bit_array(most_common);

    println!("gamma rate: {}", res);
    println!("epsilon rate: {}", !res & mask);
    println!("final result: {}", res * (!res & mask));

    // part 2
    let mut oxygen = input.clone();
    for i in (0..NUML).rev() {
        let common = most_common_bit(&oxygen, i as u32);
        oxygen = oxygen.into_iter().filter(|num| bit_at(*num, i as u32) == common).collect();
        if oxygen.len() <= 1 { break; }
    }

    let mut co2 = input.clone();
    for i in (0..NUML).rev() {
        let common = !most_common_bit(&co2, i as u32);
        co2 = co2.into_iter().filter(|num| bit_at(*num, i as u32) == common).collect();
        if co2.len() <= 1 { break; }
    }
    
    println!("oxygen gen rating: {}", oxygen[0]);
    println!("co2 rating: {}", co2[0]);
    println!("final result: {}", oxygen[0] * co2[0]);

    Ok(())
}

fn u32_from_n_bit_array<const N: usize>(bits: [u32; N]) -> u32 {
    let mut base = 0;
    for (i, bit) in bits.iter().enumerate() {
        base |= bit << i
    }
    base
}

// true for 1, false for 0
fn bit_at(num: u32, idx: u32) -> bool {
    (num & (1 << idx)) != 0
}

// true for 1, false for 0
fn most_common_bit(nums: &[u32], idx: u32) -> bool {
    let mut count = 0;
    for num in nums {
        if bit_at(*num, idx) {
            count += 1;
        }
    }

    nums.len() - count <= count
}
