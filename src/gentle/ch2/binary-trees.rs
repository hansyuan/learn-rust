extern crate rand;
use rand::prelude::*;

// back to cse 130 :D

type MaybeNode<T> = Option<Box<Node<T>>>;

// #[derive(Debug)]
struct Node<T> {
    // basic binary tree
    val: T,
    left: MaybeNode<T>,
    right: MaybeNode<T>,
}

// was trying to make the fmt recursive. i'll come back later.
impl<T: std::fmt::Display> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(ref left) = self.left {
            println!("{:#?}", left);
        }
        write!(f, "{:?}", self.val);
        if let Some(ref right) = self.right {
            println!("{:#?}", right);
        }
    }
}

impl<T: PartialOrd> Node<T> {
    // Create a new node with no children
    fn new(val: T) -> Node<T> {
        Node {
            val: val,
            left: None,
            right: None,
        }
    }

    // have a hunch, but not entirely sure
    fn boxer(node: Node<T>) -> MaybeNode<T> {
        Some(Box::new(node))
    }

    // setters
    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, val: T) {
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

        // println!("{}", self.val);

        if let Some(ref right) = self.right {
            right.traverse();
        }
    }
}

fn main() {
    // println!("Hello;");
    let mut rng = rand::thread_rng();
    let mut root = Node::new(rng.gen_range(10000, 15000));
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
        root.insert(rng.gen_range(10000, 15000));
    }

    println!("binary-tree:\n{:#?}", root);
    println!("");

    root.traverse();

    // Apparently some pointers are smart, others need to be explicit.
}
