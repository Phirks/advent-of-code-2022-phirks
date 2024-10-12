use std::fs::File;
use std::io::{BufRead,BufReader};

enum Move{
    Rock,
    Paper,
    Scissors,
}

struct Round{
    opponent: Move,
    player: Move,
}

impl Round {
    fn new(line: String) -> Self {
        let opponent = line.clone().chars().collect::<Vec<char>>()[0];
        let player = match &line[..] {
            "C Z"|"A Y"|"B X" => 'X',
            "A Z"|"B Y"|"C X" => 'Y',
            "B Z"|"C Y"|"A X" => 'Z',
            _=>'Z',
        };
        let opponent_move = match opponent {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _=> Move::Rock,
        };
        let player_move = match player {
           'X' => Move::Rock,
           'Y' => Move::Paper,
           'Z' => Move::Scissors,
           _=> Move::Rock,
        };
        Round{
            opponent: opponent_move,
            player: player_move,
        }
    }
    fn play(&self) -> (i32,i32){
        match self.opponent {
            Move::Rock => {
                match self.player {
                    Move::Rock => (1+3,1+3),
                    Move::Paper => (1+0,2+6),
                    Move::Scissors => (1+6,3+0),
                }
            }
            Move::Paper => {
                match self.player {
                    Move::Rock => (2+6,1+0),
                    Move::Paper => (2+3,2+3),
                    Move::Scissors => (2+0,3+6),
                }
            }
            Move::Scissors => {
                match self.player {
                    Move::Rock => (3+0,1+6),
                    Move::Paper => (3+6,2+0),
                    Move::Scissors => (3+3,3+3),
                }
            }
        }
    }
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut total = 0;
    for line in reader.lines() {
        let a = Round::new(line.unwrap());
        total += a.play().1;

    }
    println!("{total}");
    // let a = Round::new("A X".to_string());
    // println!("{}",a.play().1);

}
