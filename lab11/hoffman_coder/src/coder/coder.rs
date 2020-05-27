use std::collections::HashMap;
use super::generator::Node;

pub fn encode(input: Vec<u8>, h: &HashMap<u8, String>) -> String {
    let mut effect= "".to_string();
    let mut t : Option<&String>;

    for element in input {
        t = h.get(&element);
        effect.push_str(t.unwrap());
    }
    effect
}

pub fn decode(s: &str, root: &Box<Node>) -> Vec<u8> {

    let mut decoded : Vec<u8> = Vec::new();
    let mut nodeptr = root;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.value {
            decoded.push(ch);
            nodeptr = root;
        }
    }
    decoded
}