extern crate rand;
use rand::prelude::*;

// back to cse 130 :D

type MaybeNode = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    // basic binary tree
    val: i64,
    left: MaybeNode,
    right: MaybeNode,
}

impl Node {
    // Create a new node with no children
    fn new(val: i64) -> Node {
        Node {
            val: val,
            left: None,
            right: None,
        }
    }

    // have a hunch, but not entirely sure
    fn boxer(node: Node) -> MaybeNode {
        Some(Box::new(node))
    }

    // setters
    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, val: i64) {
        if val < self.val {
            //Check if there are children
            match self.left {
                Some(ref mut node) => node.insert(val),
                None => self.set_left(Self::new(val)),
            }
        } else if val > self.val {
            match self.right {
                Some(ref mut node) => node.insert(val),
                None => self.set_right(Self::new(val)),
            }
        }
    }

    fn traverse(&self) {
        // left, middle, and then right

        if let Some(ref left) = self.left {
            left.traverse();
        }

        println!("{}", self.val);

        if let Some(ref right) = self.right {
            right.traverse();
        }
    }
}

fn main() {
    // println!("Hello;");
    let mut rng = rand::thread_rng();
    let mut root = Node::new(rng.gen::<i64>());
    // root.set_left(Node::new(2));
    // root.set_right(Node::new(3));

    // root.insert(3);
    // root.insert(6);
    // root.insert(9);
    // root.insert(8);
    // root.insert(5);
    // root.insert(2);

    // root.insert(58);
    // root.insert(25);
    // root.insert(42);

    // root.insert(7);
    // root.insert(3);
    // root.insert(4);

    for _ in 0..13 {
        root.insert(rng.gen::<i64>());
    }

    println!("binary-tree:\n{:#?}", root);
    println!("");

    root.traverse();

    // Apparently some pointers are smart, others need to be explicit.
}
