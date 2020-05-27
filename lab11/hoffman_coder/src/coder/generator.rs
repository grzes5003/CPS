use std::collections::HashMap;
use std::cmp::Ordering;
use std::any::Any;
use std::fmt::Error;
use std::io::Bytes;

#[derive(Debug, Eq)]
pub struct Node {
    // childes pointers
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    // value
    pub value: Option<u8>,
    // probability
    pub prob: Option<u8>,
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


pub fn generate_tree(code_hashmap: HashMap<u8, u8>) -> Box<Node> {
    let mut tree_vec: Vec<Box<Node>> =
        code_hashmap.iter().map(|(x, y)| Box::new(Node {
            value: Some(*x),
            prob: Some(*y),
            left: None,
            right: None,
        })).collect();

    while tree_vec.len() > 1 {
        tree_vec.sort_by(|a, b| b.cmp(a));

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
    tree_vec.pop().unwrap()
}

pub fn generate_code(root: &Box<Node>, code_map: &mut HashMap<u8, String>, s: String) {
    if let Some(value) = root.value {
        code_map.insert(value, s);
        return;
    }

    if let Some(ref l) = root.left {
        generate_code(l, code_map, (s.clone() + "0"));
    }
    if let Some(ref r) = root.right {
        generate_code(r, code_map, (s.clone() + "1"));
    }
}