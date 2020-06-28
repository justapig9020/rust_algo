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
    println!("Len: {}", list.len());
    match list.remove(1) {
        Ok(val) => println!("Removed {}", val),
        Err(msg) => println!("Remove failed with: {}", msg),
    }

    list.traversal();
    println!("Len: {}", list.len());
    let rm_val = get_int();
    match list.remove_val(rm_val) {
        Ok(num) => println!("Remove @ position {}", num),
        Err(msg) => println!("Remove failed with: {}", msg),
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
    T: std::fmt::Display + Copy + PartialEq,
{
    fn new() -> Box<List<T>> {
        Box::new(List {
            head: None,
            size: 0,
        })
    }

    fn insert(&mut self, val: T) {
        let mut ptr = &mut self.head;
        loop {
            match ptr {
                Some(ref mut this) => ptr = &mut this.next,
                None => break,
            }
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
            match ptr {
                Some(cur) => ptr = &mut cur.next,
                None => return Err("Over size while traversaling".to_string()),
            }
        }

        match ptr {
            Some(n) => {
                let ret = n.val;
                *ptr = n.next.take();
                self.size -= 1;
                return Ok(ret);
            }
            None => return Err("Node not exist".to_string()),
        }
    }

    fn remove_val(&mut self, val: T) -> Result<usize, String> {
        let mut ptr = &mut self.head; // &mut Option<Box<Node>>
        let mut num = 0;
        loop {
            match ptr {
                Some(cur) if cur.val == val => {
                    // &cur
                    *ptr = cur.next.take();
                    self.size -= 1;
                    return Ok(num);
                }
                Some(cur) => {
                    // &mut cur
                    ptr = &mut cur.next;
                }
                None => return Err("No node with this value".to_string()),
            }
            num += 1;
        }
    }

    /*
    fn remove_val(&mut self, val: T) -> Result<usize, String> {
        let mut ptr = &mut self.head;
        let mut idx = 0;
        loop {
            match ptr {
                Some(cur) => {
                    if cur.val == val {
                        break;
                    } else {
                        ptr = &mut cur.next;
                        idx += 1;
                    }
                }
                None => {
                    return Err("Value not found".to_string());
                }
            }
        }
        *ptr = ptr.unwrap().next.take();
        return Ok(idx);
    }
    */

    fn traversal(&self) {
        let mut ptr = &self.head;
        while ptr.is_some() {
            let node = ptr.as_ref().unwrap();
            print!("{}, ", node.val);
            ptr = &ptr.as_ref().unwrap().next;
        }
        println!("");
    }

    fn len(&self) -> usize {
        self.size
    }
}
