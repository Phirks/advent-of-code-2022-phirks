use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut elf_tracker: Vec<i32> = vec![0];
    let mut calorie_counter: i32 = 0;
    for line in reader.lines() {
        if let Ok(ref uline) = line {
            if uline == "" {
                elf_tracker.push(calorie_counter);
                calorie_counter = 0;
            } else {
                calorie_counter += uline.parse::<i32>().unwrap();
            }
        } else {
        }
    }
    elf_tracker.sort();
    let final_vec = elf_tracker.into_iter().rev().collect::<Vec<i32>>();
    println!("{}", final_vec[0] + final_vec[1] + final_vec[2]);
}
