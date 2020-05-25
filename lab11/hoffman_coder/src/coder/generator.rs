use std::collections::HashMap;
use std::cmp::Ordering;
use std::any::Any;
use std::fmt::Error;

#[derive(Debug, Eq)]
pub struct Node {
    // childes pointers
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

    // value
    value: Option<u8>,
    // probability
    prob: Option<u8>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prob.cmp(&other.prob)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.prob == other.prob
    }
}


pub fn generate_tree(code_hashmap: HashMap<u8, u8>) -> Vec<Box<Node>> {
    let mut tree_vec: Vec<Box<Node>> =
        code_hashmap.iter().map(|(x, y)| Box::new(Node {
            value: Some(*x),
            prob: Some(*y),
            left: None,
            right: None,
        })).collect();

    while tree_vec.len() > 1 {
        tree_vec.sort_by(|a, b| a.cmp(b));

        let x = tree_vec.pop().unwrap();
        let y = tree_vec.pop().unwrap();

        let z = Box::new(Node {
            value: None,
            prob: match (x.prob, y.prob) {
                (Some(m), Some(n)) => Some(m + n),
                _ => None,
            },
            left: Some(x),
            right: Some(y),
        });
        tree_vec.push(z);
    }
    tree_vec
}