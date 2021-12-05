use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

type Point = (i32, i32);

#[derive(Debug)]
struct Segment(Point, Point);

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let coords: Vec<_> = line.split("->").map(|t| t.split(",")).flatten().map(|n| n.trim().parse::<i32>().unwrap()).collect();
            Segment((coords[0], coords[1]), (coords[2], coords[3]))
        })
        .collect::<Vec<_>>();

    let mut hm: HashMap<Point, u32> = HashMap::new();

    // true for part 2
    let diag = true;

    for segment in input {
        let diff = ((segment.1.0 - segment.0.0).signum(), (segment.1.1 - segment.0.1).signum());

        if !diag && diff.0 != 0 && diff.1 != 0 {
            continue;
        }

        let mut curr = segment.0;

        while curr != segment.1 {
            let entry = hm.entry(curr).or_insert(0);
            *entry += 1;
            curr.0 += diff.0;
            curr.1 += diff.1;
        }
        // insert endpointw
        let entry = hm.entry(curr).or_insert(0);
        *entry += 1;
    }

    println!("{:?}", hm.values().filter(|v| **v > 1).count());

    Ok(())
}
