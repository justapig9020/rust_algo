fn main() {
    println!("Hello, world!");
}

enum Mid{
    equal(Node),
    end(String),
}

struct Node {
    val: u8,
    less: Option<Box<Node>>,
    great: Option<Box<Node>>,
    equal: Option<Box<Mid>>,
}

impl Node {
    fn new(val: u8) -> Box<Node> {
        Box::new(Node {
            val: val,
            less: None,
            great: None,
            equal: None,
        })
    }
}

struct Tst {
    root: Option<Node>,
}

impl Tst {
    fn new() -> Box<Tst> {
        Box::new(Tst {
            root: None,
        })
    }

    fn insert(&mut self, s: String) {
        
    }
}
