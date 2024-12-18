use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Enumerate;
use std::ops::Deref;
use std::panic::Location;

#[derive(Clone, Copy, Debug)]
struct Tree {
    left: Option<u8>,
    right: Option<u8>,
    top: Option<u8>,
    bottom: Option<u8>,
    height: u8,
    location: (usize, usize),
    hidden: Option<bool>,
    complete: bool,
    visible_trees: Option<i32>,
}

impl Tree {
    fn new(height: u8, location: (usize, usize)) -> Tree {
        Tree {
            left: None,
            right: None,
            top: None,
            bottom: None,
            height: height,
            location: location,
            hidden: None,
            complete: false,
            visible_trees: None,
        }
    }

    fn populate(
        &mut self,
        left: Option<u8>,
        right: Option<u8>,
        top: Option<u8>,
        bottom: Option<u8>,
    ) {
        //The following pulls the data from the neighboring trees to find out if it's hidden
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        if self.left.is_some()
            && self.right.is_some()
            && self.top.is_some()
            && self.bottom.is_some()
        {
            self.hidden = Some(
                self.left.unwrap() >= self.height
                    && self.right.unwrap() >= self.height
                    && self.top.unwrap() >= self.height
                    && self.bottom.unwrap() >= self.height,
            );
        }
        // println!("{:?}",self)
    }
}

fn pull(
    left: Option<Tree>,
    right: Option<Tree>,
    top: Option<Tree>,
    bottom: Option<Tree>,
) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
    //The following pulls the data from the neighboring trees to find out if it's hidden
    let mut left_value = None;
    let mut right_value = None;
    let mut top_value = None;
    let mut bottom_value = None;
    if let Some(left_tree) = left {
        let height = left_tree.height.clone();
        if let Some(left_tree_vision) = left_tree.left {
            if left_tree_vision > height {
                left_value = Some(left_tree_vision);
            } else {
                left_value = Some(height);
            }
        } else {
            left_value = None;
        }
    } else {
        left_value = Some(0);
    }
    if let Some(right_tree) = right {
        let height = right_tree.height.clone();
        if let Some(right_tree_vision) = right_tree.right {
            if right_tree_vision > height {
                right_value = Some(right_tree_vision);
            } else {
                right_value = Some(height);
            }
        } else {
            right_value = None;
        }
    } else {
        right_value = Some(0);
    }
    if let Some(top_tree) = top {
        let height = top_tree.height.clone();
        if let Some(top_tree_vision) = top_tree.top {
            if top_tree_vision > height {
                top_value = Some(top_tree_vision);
            } else {
                top_value = Some(height);
            }
        } else {
            top_value = None;
        }
    } else {
        top_value = Some(0);
    }
    if let Some(bottom_tree) = bottom {
        let height = bottom_tree.height.clone();
        if let Some(bottom_tree_vision) = bottom_tree.bottom {
            if bottom_tree_vision > height {
                bottom_value = Some(bottom_tree_vision);
            } else {
                bottom_value = Some(height);
            }
        } else {
            bottom_value = None;
        }
    } else {
        bottom_value = Some(0);
    }
    (left_value, right_value, top_value, bottom_value)
}

fn grab_tree(
    forest: Vec<Vec<Tree>>,
    i: usize,
    j: usize,
) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
    let left_tree = if j > 0 {
        Some(forest[i][j - 1].clone())
    } else {
        None
    };
    let right_tree = if j < forest[0].len() - 1 {
        Some(forest[i][j + 1].clone())
    } else {
        None
    };
    let top_tree = if i > 0 {
        Some(forest[i - 1][j].clone())
    } else {
        None
    };
    let bottom_tree = if i < forest.len() - 1 {
        Some(forest[i + 1][j].clone())
    } else {
        None
    };
    pull(left_tree, right_tree, top_tree, bottom_tree)
}

fn parse_tree(forest: &mut Vec<Vec<Tree>>, i: usize, j: usize) -> i32 {
    let (left, right, top, bottom) = grab_tree(forest.clone(), i, j);
    forest[i][j].populate(left, right, top, bottom);
    if i == 0 || j == 0 || i == forest.len() - 1 || j == forest[0].len() - 1 {
        forest[i][j].hidden = Some(false);
    }
    //forest[i][j].right.unwrap_or(0) as i32
    if let Some(a) = forest[i][j].hidden {
        if a {
            1
        } else {
            0
        }
    } else {
        2
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut forest: Vec<Vec<Tree>> = vec![];
    for (i, line) in reader.lines().enumerate() {
        let mut tree_line: Vec<Tree> = vec![];
        for (j, char) in line.unwrap().chars().enumerate() {
            let mut tree = Tree::new(char.to_digit(11).unwrap() as u8, (i, j));
            tree_line.push(tree);
        }
        forest.push(tree_line);
    }
    let ilen = forest.len();
    let jlen = forest[0].len();
    let mut max = 0;

    for i in 0..ilen {
        for j in 0..jlen {
            let (mut l, mut r, mut t, mut b) = (0, 0, 0, 0);
            let mut height = 0;
            {
                height = forest[i][j].height;
            }
            if j > 0 {
                for jl in (0..j).rev() {
                    l += 1;
                    if forest[i][jl].height >= height {
                        break;
                    }
                }
            }
            if j < jlen {
                for jr in j + 1..jlen {
                    r += 1;
                    if forest[i][jr].height >= height {
                        break;
                    }
                }
            }
            if i < ilen {
                for ib in i + 1..ilen {
                    b += 1;
                    if forest[ib][j].height >= height {
                        break;
                    }
                }
            }
            if i > 0 {
                for it in (0..i).rev() {
                    //println!("loop {} - {} >= {}", it, forest[it][j].height, height);
                    t += 1;
                    if forest[it][j].height >= height {
                        break;
                    }
                }
            }
            let senic_score = l * r * t * b;
            if max < senic_score {
                max = senic_score;
            }
        }
        //print!("\n");
    }
    println!("{}", max);
    //println!("{:?}",forest[0][0].clone())
}
