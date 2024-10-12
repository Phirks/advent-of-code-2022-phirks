use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let char_array = line.unwrap().clone();
        let mut char_buffer = vec![
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        let mut answer: usize = 0;
        for (i, char) in char_array.chars().enumerate() {
            let index = i % 14;
            print!("{char}");
            char_buffer[index] = char;
            let mut char_buffer_sorted = char_buffer.clone();
            char_buffer_sorted.sort();
            let mut char_match = ' ';
            let mut char_found = false;
            for char in char_buffer_sorted {
                if char_match == char {
                    char_found = true;
                }
                char_match = char;
            }
            if !char_found && i > 14 {
                answer = i + 1;
                break;
            }
        }
        println!("{answer}");
    }
}
