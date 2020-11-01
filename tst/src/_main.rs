use std::io;
fn main() {
    let mut list = List::new();
    loop {
        let s = read_line();
        if s.trim() == "exit" {
            break;
        }
        list.insert(s);
    }
    list.print();
    loop {
        let s = read_line();
        if s.trim() == "exit" {
            break;
        }
        if let Ok(n) = list.remove(&s) {
            println!("Remove val: {} @ {}", s, n);
            println!("Current list:");
            list.print();
        } else {
            println!("Val: {} not found", s);
        }
    }
    list.print();
}

fn read_line() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read line");
    ret.trim().to_string()
}

struct List {
    head: Option<Box<Node>>,
    size: usize,
}

impl List {
    fn new() -> Box<List> {
        Box::new(List {
            head: None,
            size: 0,
        })
    }

    fn print(&self) {
        let mut ptr = &self.head;
        let mut i = 0;
        while let Some(curr) = ptr {
            println!("{}: {}", i, curr.val);
            ptr = &curr.next;
            i += 1;
        }
    }
    
    fn insert(&mut self, val: String) {
        let mut ptr = &mut self.head;
        while let Some(curr) = ptr {
            ptr = &mut curr.next;
        }
        *ptr = Some(Node::new(val));
        self.size += 1;
    }

    fn remove(&mut self, tar: &String) -> Result<i32, ()>{
        let mut ptr = &mut self.head;
        let mut i = 0;
        while let Some(curr) = ptr {
            if curr.val == *tar {
                *ptr = curr.next.take();
                return Ok(i);
            }
            //ptr = &mut curr.next;
            ptr = &mut ptr.as_mut().unwrap().next;
            i += 1;
        }
        Err(())
    }
}

struct Node {
    val: String,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: String) -> Box<Node> {
        Box::new(Node {
            val: val,
            next: None,
        })
    }
}
