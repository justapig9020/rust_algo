use std::io;
use std::num::ParseIntError;

fn main() {
    let mut root: Option<Box<Node>> = None;
    loop {
        let val = get_int();
        if val == 0 {
            break;
        }
        match root {
            Some(ref mut r) => r.insert(val),
            None => root = Some(Node::new(val)),
        }
    }

    match root {
        Some(ref r) => r.traversal(),
        None => println!("Tree is empty"),
    }
}

fn get_int_rst() -> Result<i32, ParseIntError> {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    let ret: i32 = ret.trim().parse()?;
    Ok(ret)
}

fn get_int() -> i32 {
    loop {
        match get_int_rst() {
            Ok(ret) => return ret,
            Err(_) => println!("Please input a int"),
        }
    }
}

struct Node {
    val: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node {
            val: val,
            right: None,
            left: None,
        })
    }

    fn insert(&mut self, val: i32) {
        let update = |next: &mut Option<Box<Node>>, v: i32| match next {
            Some(ref mut n) => n.insert(v),
            None => *next = Some(Node::new(v)),
        };
        if val > self.val {
            update(&mut self.right, val);
        } else {
            update(&mut self.left, val);
        }
    }

    // Not good version
    fn insert_(&mut self, val: i32) {
        if val > self.val {
            match self.right {
                Some(ref mut next) => next.insert_(val),
                None => self.right = Some(Node::new(val)),
            }
        } else {
            match self.left {
                Some(ref mut next) => next.insert_(val),
                None => self.left = Some(Node::new(val)),
            }
        }
    }

    fn traversal(&self) {
        self.preorder();
        println!("");
    }

    fn preorder(&self) {
        print!("({})", self.val);
        match self.left {
            Some(ref next) => next.preorder(),
            None => (),
        }
        match self.right {
            Some(ref next) => next.preorder(),
            None => (),
        }
        print!(")");
    }
}
