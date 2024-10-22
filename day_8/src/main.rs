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
        }
    }

    fn pull(
        &mut self,
        left: Option<Tree>,
        right: Option<Tree>,
        top: Option<Tree>,
        bottom: Option<Tree>,
    ) {
        //The following pulls the data from the neighboring trees to find out if it's hidden
        if self.hidden.is_none() {
            if let Some(left_tree) = left {
                let height = left_tree.height.clone();
                if let Some(left_tree_vision) = left_tree.left {
                    if left_tree_vision > height {
                        self.left = Some(left_tree_vision);
                    } else {
                        self.left = Some(height);
                    }
                } else {
                    self.left = Some(height);
                }
            } else {
                self.left = Some(0);
                println!("{:?}",self)
            }
            if let Some(right_tree) = right {
                let height = right_tree.height.clone();
                if let Some(right_tree_vision) = right_tree.left {
                    if right_tree_vision > height {
                        self.right = Some(right_tree_vision);
                    } else {
                        self.right = Some(height);
                    }
                } else {
                    self.right = Some(height);
                }
            } else {
                self.right = Some(0);
            }
            if let Some(top_tree) = top {
                let height = top_tree.height.clone();
                if let Some(top_tree_vision) = top_tree.left {
                    if top_tree_vision > height {
                        self.top = Some(top_tree_vision);
                    } else {
                        self.top = Some(height);
                    }
                } else {
                    self.top = Some(height);
                }
            } else {
                self.top = Some(0);
            }
            if let Some(bottom_tree) = bottom {
                let height = bottom_tree.height.clone();
                if let Some(bottom_tree_vision) = bottom_tree.left {
                    if bottom_tree_vision > height {
                        self.bottom = Some(bottom_tree_vision);
                    } else {
                        self.bottom = Some(height);
                    }
                } else {
                    self.bottom = Some(height);
                }
            } else {
                self.bottom = Some(0);
            }
        }
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
        println!("{:?}",self)
    }
}

fn grab_tree(forest: &mut Vec<Vec<Tree>>,i: usize,j: usize){
    let left_tree = if i>0 {
        Some(forest[i-1][j].clone())
    } else {
        None
    };
    let right_tree = if i<20 {
        Some(forest[i+1][j].clone())
    } else {
        None
    };
    let top_tree = if j>0 {
        Some(forest[i][j-1].clone())
    } else {
        None
    };
    let bottom_tree = if j<20 {
        Some(forest[i][j+1].clone())
    } else {
        None
    };

    
    let mut target_tree = forest[i][j];
    target_tree.pull(left_tree, right_tree, top_tree, bottom_tree);
    println!("{:?}",target_tree);

}


fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut forest: Vec<Vec<Tree>> = vec![];
    for (i, line) in reader.lines().enumerate() {
        let mut tree_line: Vec<Tree> = vec![];
        for (j, char) in line.unwrap().chars().enumerate() {
            let mut tree = Tree::new(
                char.to_digit(11).unwrap() as u8,
                (i, j),
            );
            tree_line.push(tree);
        }
        forest.push(tree_line);
    }
    forest = grab_tree(&mut forest, 0, 0);
    println!("{:?}",forest[0][0].clone())
}
