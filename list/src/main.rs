use std::io;
use std::num::ParseIntError;

fn main() {
    let mut list = List::<i32>::new();
    loop {
        let val = get_int();
        if val == 0 {
            break;
        }
        list.insert(val);
    }
    list.traversal();
    match list.remove(1) {
        Ok(num) => println!("Removed {}", num),
        Err(msg) => println!("Remove failed with {}", msg),
    }
    list.traversal();
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

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Node<T> {
    fn new(val: T) -> Box<Node<T>> {
        Box::new(Node {
            val: val,
            next: None,
        })
    }
}

impl<T> List<T>
where
    T: std::fmt::Display + std::fmt::Debug + Copy + Clone,
{
    fn new() -> Box<List<T>> {
        Box::new(List {
            head: None,
            size: 0,
        })
    }

    fn insert(&mut self, val: T) {
        let mut ptr = &mut self.head;
        while ptr.is_some() {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        *ptr = Some(Node::new(val));
        self.size += 1;
    }

    fn remove(&mut self, num: usize) -> Result<T, String> {
        if num >= self.size {
            return Err("Over size".to_string());
        }

        let mut ptr = &mut self.head;
        for _ in 0..num {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        //let ret = ptr.unwrap().val.take();
        match ptr {
            Some(n) => {
                let ret = n.val;
                *ptr = n.next.take();
                return Ok(ret);
            }
            None => return Err("Empty node".to_string()),
        }
    }

    fn traversal(&self) {
        let mut ptr = &self.head;
        while ptr.is_some() {
            let node = ptr.as_ref().unwrap();
            print!("{}, ", node.val);
            ptr = &ptr.as_ref().unwrap().next;
        }
        println!("");
    }
}
