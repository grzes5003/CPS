use core::borrow::BorrowMut;
use std::collections::HashMap;

mod coder;

fn main() {
    let file_path = "../x";

    println!("Importing");
    let mut my_vec = Vec::new();
    match coder::importer::import_code(String::from(file_path), my_vec.borrow_mut()) {
        Ok(_) => println!("{:?}", my_vec),
        _ => println!("Cannot read/load file")
    }

    let eff = coder::importer::vec2hashmap(my_vec.clone());
    println!("{:?}", eff);

    let mut root= coder::generator::generate_tree(eff);

    let mut code : HashMap<u8, String> = HashMap::new();
    coder::generator::generate_code(&root, &mut code, "".to_string());

    println!("{:?}", code);

    println!("input vector {:?}", my_vec);

    let encoded = coder::coder::encode(my_vec, &code);
    println!("{:?}", encoded);

    let decoded = coder::coder::decode(encoded.as_str(), &root);
    println!("output vector {:?}", decoded);


    // przeklamanie
    let mut changed : String =
        encoded.chars().enumerate().map(|(x,y)| if x!=4 {y} else { if y=='1' {'0'} else {'1'} }).collect();

    let decoded2 = coder::coder::decode(changed.as_str(), &root);
    println!("changed vector {:?}", decoded2);

}
