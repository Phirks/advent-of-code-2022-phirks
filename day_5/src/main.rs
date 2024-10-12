use std::fs::File;
use std::io::{BufRead,BufReader};

struct Piles {
    pile1: Vec<char>,
    pile2: Vec<char>,
    pile3: Vec<char>,
    pile4: Vec<char>,
    pile5: Vec<char>,
    pile6: Vec<char>,
    pile7: Vec<char>,
    pile8: Vec<char>,
    pile9: Vec<char>,
}

impl Piles {
    fn new() -> Piles{
        Piles{
            // pile1: vec!['Z','N'],
            // pile2: vec!['M','C','D'],
            // pile3: vec!['P'],
            pile1: vec!['D','B','J','V'],
            pile2: vec!['P','V','B','W','R','D','F'],
            pile3: vec!['R','G','F','L','D','C','W','Q'],
            pile4: vec!['W','J','P','M','L','N','D','B'],
            pile5: vec!['H','N','B','P','C','S','Q'],
            pile6: vec!['R','D','B','S','N','G'],
            pile7: vec!['Z','B','P','M','Q','F','S','H'],
            pile8: vec!['W','L','F'],
            pile9: vec!['S','V','F','M','R'],
        }
    }
    fn push_val(&mut self,value: char, to: i32){
        assert!(to > 0);
        assert!(to < 10);
        match to{
            1 => self.pile1.push(value),
            2 => self.pile2.push(value),
            3 => self.pile3.push(value),
            4 => self.pile4.push(value),
            5 => self.pile5.push(value),
            6 => self.pile6.push(value),
            7 => self.pile7.push(value),
            8 => self.pile8.push(value),
            9 => self.pile9.push(value),
            _ => panic!(),
        };
    }
    fn pop_val(&mut self,from: i32) -> char{
        assert!(from > 0);
        assert!(from <10);
        match from{
            1 => self.pile1.pop().unwrap().clone(),
            2 => self.pile2.pop().unwrap().clone(),
            3 => self.pile3.pop().unwrap().clone(),
            4 => self.pile4.pop().unwrap().clone(),
            5 => self.pile5.pop().unwrap().clone(),
            6 => self.pile6.pop().unwrap().clone(),
            7 => self.pile7.pop().unwrap().clone(),
            8 => self.pile8.pop().unwrap().clone(),
            9 => self.pile9.pop().unwrap().clone(),
            _ => panic!(),
        }
    }
        fn do_calc(&mut self,num: i32, from: i32, to: i32){
            let mut a: Vec<char> = vec![];
            for _ in 0..num { 
                a.push(self.pop_val(from));
            }
            for _ in 0..num {
                self.push_val(a.pop().unwrap(), to);
            }
        }

}

fn main() {
    
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut a = Piles::new();
    for line in reader.lines() {
        let line = line.unwrap().clone();
        let split_line: Vec<&str> = line.split(" ").collect();
        let (num,from,to) = (split_line[1],split_line[3],split_line[5]);
        a.do_calc(num.parse::<i32>().unwrap(), from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap());   
        }
    println!("{}{}{}{}{}{}{}{}{}",
    a.pile1[a.pile1.len()-1],
    a.pile2[a.pile2.len()-1],
    a.pile3[a.pile3.len()-1],
    a.pile4[a.pile4.len()-1],
    a.pile5[a.pile5.len()-1],
    a.pile6[a.pile6.len()-1],
    a.pile7[a.pile7.len()-1],
    a.pile8[a.pile8.len()-1],
    a.pile9[a.pile9.len()-1],
    );

    }