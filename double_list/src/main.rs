use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let node = Rc::new(RefCell::new(Node {
        val: 10,
        prev: None,
        next: None,
    }));
    let nextNode = Rc::new(RefCell::new(Node {
        val: 11,
        prev: None,
        next: None,
    }));
    node.borrow_mut().next = Some(nextNode.clone());
    nextNode.borrow_mut().prev = Some(node.clone());
}

struct Node {
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct List {
    head: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val: val,
            prev: None,
            next: None,
        }))
    }
}

impl List {
    fn new() -> List {
        List { head: None }
    }

    fn insert(&mut self, val: i32) {
        let mut ptr = &mut self.head;
        // &Option<Rc<RefCell<Node>>>
        // ptr -> Option-Rc -> RefCell-Node-Option
        // ptr ------------------------------^
        //
        // Rc::new(5)
        // Rc::clone()
        // Rc -> i32(5)
        // Rc ---^
        while let Some(cur) = ptr {
            // cur: Rc<RefCell<Node>>
            // cur.clone(): Rc<RefCell<Node>
            // cur.borro(): &Node
            // Node.next
            ptr = &mut cur.borrow_mut().next;
            // cur.clone()
        }
        // ptr

        *ptr = Some(Node::new(val));
    }
}
