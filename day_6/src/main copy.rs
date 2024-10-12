use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let char_array = line.unwrap().clone();
        let mut char_buffer = vec![' ', ' ', ' ', ' '];
        let mut answer: usize = 0;
        for (i, char) in char_array.chars().enumerate() {
            let index = i % 4;
            print!("{char}");
            char_buffer[index] = char;
            let (v1, v2, v3, v4) = (
                char_buffer[0],
                char_buffer[1],
                char_buffer[2],
                char_buffer[3],
            );
            if i < 4 || v1 == v2 || v1 == v3 || v1 == v4 || v2 == v3 || v2 == v4 || v3 == v4 {
            } else {
                answer = i + 1;
                break;
            }
        }
        println!("{answer}");
    }
}
