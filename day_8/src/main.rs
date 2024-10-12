use std::fs::File;
use std::io::{BufRead,BufReader};

struct Tree{
    left: Option<u8>,
    right: Option<u8>,
    top: Option<u8>,
    bottom: Option<u8>,
    height: u8,
    location: (i32,i32),
    hidden: Option<bool>,
    complete: bool,
}

impl Tree{
    fn new(height:u8,location:(i32,i32)) -> Tree{
        Tree{
            left: None,
            right: None,
            top: None,
            bottom: None,
            height: height,
            location: location,
            hidden: None,
            complete: false,
        }

    }

    fn pull(&self)-> bool{
        if self.hidden != None{
            


        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}",line.unwrap());
    }
}

