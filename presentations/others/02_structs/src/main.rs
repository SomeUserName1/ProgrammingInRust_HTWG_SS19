use std::f32::consts::PI as PI;

#[derive(Debug)]
struct Circle(f32); // radius
impl Circle { 
    fn new(radius: f32) -> Self {
        Self(radius) 
    } 

    fn area(&self) -> f32 {
        2.0 * PI * self.0
    }

    fn from_rect(r: Rectangle) -> Self {
        Self((r.0 * r.1) / (2.0 * PI))
    }
}


#[derive(Debug)]
struct Triangle(f32, f32); // height, width
impl Triangle {
    fn new(height: f32, width: f32) -> Self {
        Self(height, width)
    }

    fn area(&self) -> f32 {
        0.5 * self.0 * self.1
    }
}

#[derive(Debug)]
struct Rectangle(f32, f32);
impl Rectangle {
    fn new(height: f32, width: f32) -> Self {
        Self(height, width)
    }

    fn area(&self) -> f32 {
        self.0 * self.1
    }
}

#[derive(Debug)]
struct Node<'a><T: Ord> {
    id: i32,
    value: Option<T: Ord>,
    children: mut Vec<Option<&'a Self<'a>>>,
}
impl Node {
    fn new(id: i32, value: Option<T>, children: Vec<Option<&'a Self<'a><T: Ord>>>) -> Self {
        Node {
            id,
            value,
            children
        }
    }
}

[#derive(Debug)]
struct BTree<'a><T: Ord> {
    root: Node<'a><T: Ord>,
    id_count: mut i32,
    split_rate: i32
}
impl BTree {
    fn new() -> Self {
        Btree {
            root: Node::new(0, Option::None ,Vec::new());
        }
    }

    fn insert(value: Option<T: Ord>) {
        match &self.root.children.len() {
            0 => {
                &self.id_count += 1;
                &self.root.children.push(Node::new(&self.id_count, value, Vec::new()));
            },
            // insert at right place
            d if d < &self.split_rate => {
                for child in &self.root.children {
                    match child.value {
                        v if v < value {continue;},
                        v if v > value {}
                        // exchange
                        // Rust linked list; too many linked lists with rust
                }
            },
            // SPLIT!!
            d if d > &self.split_rate => {}
        }
    }
}

fn main() { 
    println!("MARNO HALT DEINE DUMME SCHEIẞ FRESSE!");

    let c = Circle::new(666.0);
    let t = Triangle::new(42.0, 42.0);
    let r = Rectangle::new(12.0, 34.0);

    println!("Circle {:?}, with area {}", c, c.area());
    println!("Triangle {:?}, with area {}", t, t.area());
    println!("GetRect {:?}, with area {}", r, r.area());

    let nc = Circle::from_rect(r);

    println!("{:?} with area {}", nc, nc.area());

    let l0 = Node
}
