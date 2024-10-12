use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::vec;

struct Node{
    children: Vec<Node>,
    name: String,
    size: Cell<usize>,
    location: RefCell<String>,
}

impl Node{
    fn new(name: String, size: usize, location: String) -> Node{
        Node { 
            children: vec![], 
            name: name, 
            size: Cell::new(size),
            location: RefCell::new(location),
         }
    }
    fn add_child(&mut self,name: String, size: usize){
        self.children.push(Node::new(name.clone(), size, format!("{}/{}",self.location.borrow(),name)));
    }
    fn goto(&mut self, location: String)-> &mut Node{
        if location=="".to_string(){return self}
        let location_vec: Vec<&str>;
        if location.starts_with("/") {
            location_vec = location.split('/').collect(); 
        } else {
            location_vec = vec![&location[..]] 
        }
            
       let mut a = self;
       for step in location_vec{
                if step == "" {

                } else{
                a = a.children.iter_mut().find(|child| child.name == step).unwrap();
                }
       }
       a
    }
    fn getsize(&mut self){
        let mut size = 0;
        for child in self.children.iter_mut(){
            if child.size.get() == 0{
                child.getsize();
            } 
            size += child.size.get();
        }
        self.size.set(size);
    }
    fn find_sizes(&self) -> Vec<(usize)>{
        let mut sizes = vec![];
        for child in self.children.iter(){
            if child.children.len()>0{
                sizes.extend(child.find_sizes());
                if child.size.get() >= 6233734{
                    sizes.push(child.size.get());
                }
            }  
        }
        sizes
    }
    fn get_parent_location(&mut self) -> String{
        let location = self.location.borrow();
        let mut location_vec: Vec<&str> = location.split('/').collect(); 
        let _ = location_vec.pop();
        location_vec.join("/")

    }
    fn display_tree(&self,indent: String){
        for child in self.children.iter(){
            println!("{}{},{}",indent,child.name,child.size.get());
            if child.children.len()>0{
                child.display_tree(format!("{}{}",indent," "));
            }
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut root = Node::new("root".to_string(), 0, "".to_string());
    {
        let mut node=&mut root;
        for line in reader.lines() {
            if let Ok(ln) = line{
                match &ln[..] {
                   touch if touch.starts_with("$ cd ..") => {
                    let parent_location = node.get_parent_location();
                    node= root.goto(parent_location);
                   }
                   touch if touch.starts_with("$ cd") => {
                    node= node.goto(touch[5..].to_string());
                   }
                   touch if touch.starts_with("dir") => {
                        node.add_child(touch[4..].to_string(), 0);
                   }
                   touch if touch.starts_with("$ ls") => {}
                   touch if touch!="" => {
                    let args: Vec<&str> = touch.split(" ").collect();
                    let (size, name)=(args[0].to_string(),args[1].to_string());
                    node.add_child(name, size.parse().unwrap());
                   }
                   _=>{
                    "";

                   }

                };
            }
        }
    root.getsize();
    let mut size_total = 0;
    let mut sizes = root.find_sizes();
    sizes.sort();
    println!("{}",sizes[0]);
    for size in sizes{
        //println!("{} - {}",size.1,size.0);
    }
    
    println!("{}",root.size.get());
    //println!(" ----- ");
    //root.display_tree("".to_string());
    }
}
