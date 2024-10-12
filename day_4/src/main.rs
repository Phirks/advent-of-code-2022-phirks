use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut total = 0;
    for line in reader.lines() {
            let line_parsed: Vec<i32> = line.unwrap().clone().split(|c| c == '-' || c == ',').map(|i| i.parse().unwrap()).collect();
            let (r1,r2,r3,r4) = (line_parsed[0],line_parsed[1],line_parsed[2],line_parsed[3]);
            if r1 < r3 && r2 < r3 {
            } else if r1 > r4 && r2 > r4{
            } else{
                total += 1;
            }
        }
        println!("{total}");
    }

