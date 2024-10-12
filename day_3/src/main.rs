use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

struct Group{
    line1: Vec<char>,
    line2: Vec<char>,
    line3: Vec<char>,
}

impl Group{
    fn new(line1: &str,line2: &str,line3: &str) -> Group {
        let mut l1: Vec<char> = line1.chars().collect();
        let mut l2: Vec<char> = line2.chars().collect();
        let mut l3: Vec<char> = line3.chars().collect();
        l1.sort();
        l2.sort();
        l3.sort();
        Group{
            line1: l1,
            line2: l2,
            line3: l3,
        }
    }
    fn get_letter(&self) -> char {
        let mut repeats: Vec<char> = vec![];
        repeats = get_repeats(self.line1.clone(), self.line2.clone());
        repeats = get_repeats(repeats, self.line3.clone());
        if repeats.len()>0{
            repeats[0]
        } else {
            ' '
        }
    }
}

fn get_repeats(sorted_chars1: Vec<char>, sorted_chars2: Vec<char>) -> Vec<char>{
    let mut repeats: Vec<char> = vec![];
    let mut last: char = ' ';
    for val in sorted_chars1.iter(){
        if *val != last{
            last = *val;
            if let Some(ans) = sorted_chars2.iter().find(|v| &val==v){
                repeats.push(*ans);
            }
        }
    }
    repeats
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut repeats: Vec<char> = vec![];
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    loop{
        if lines.len()<3{break;}
        let a = Group::new(&lines.pop().unwrap()[..], &lines.pop().unwrap()[..], &lines.pop().unwrap()[..]);
        repeats.push(a.get_letter());
    }
    let translator = HashMap::from([

        ('a',1),
        ('b',2),
        ('c',3),
        ('d',4),
        ('e',5),
        ('f',6),
        ('g',7),
        ('h',8),
        ('i',9),
        ('j',10),
        ('k',11),
        ('l',12),
        ('m',13),
        ('n',14),
        ('o',15),
        ('p',16),
        ('q',17),
        ('r',18),
        ('s',19),
        ('t',20),
        ('u',21),
        ('v',22),
        ('w',23),
        ('x',24),
        ('y',25),
        ('z',26),
        ('A',27),
        ('B',28),
        ('C',29),
        ('D',30),
        ('E',31),
        ('F',32),
        ('G',33),
        ('H',34),
        ('I',35),
        ('J',36),
        ('K',37),
        ('L',38),
        ('M',39),
        ('N',40),
        ('O',41),
        ('P',42),
        ('Q',43),
        ('R',44),
        ('S',45),
        ('T',46),
        ('U',47),
        ('V',48),
        ('W',49),
        ('X',50),
        ('Y',51),
        ('Z',52),
    ]
    );
    //repeats.sort();
    let mut total = 0;
    for items in repeats.iter(){
        // println!("{}",items.clone());
        total +=translator.get_key_value(items).unwrap().1;
    }
    println!("{}",total);
}

