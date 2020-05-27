use std::io;
use std::io::prelude::*;
use std::fs::File;
use core::borrow::BorrowMut;
use std::collections::HashMap;


pub fn import_code(file_name: String, chars_vec: &mut Vec<u8>) -> io::Result<()> {
    let mut f = File::open(file_name)?;

    f.read_to_end(chars_vec.borrow_mut())?;
    Ok(())
}


pub fn vec2probability_map(chars_vec: Vec<u8>) -> Vec<(u8, f32)> {
    let mut code_hashmap = HashMap::new();

    for item in &chars_vec {
        *code_hashmap.entry(*item).or_insert(0) += 1;
    }

    let chars_vec_len = chars_vec.len() as f32;

    println!("{:?}, size: {}", code_hashmap, chars_vec_len);

    code_hashmap.iter().map(|(x,y)| (*x, (*y as f32)/chars_vec_len )).collect()
}

pub fn vec2hashmap(chars_vec: Vec<u8>) -> HashMap<u8,u16> {
    let mut code_hashmap = HashMap::new();

    for item in &chars_vec {
        *code_hashmap.entry(*item).or_insert(0) += 1;
    }
    code_hashmap
}