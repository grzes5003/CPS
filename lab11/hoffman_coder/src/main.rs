use core::borrow::BorrowMut;

mod coder;

fn main() {
    println!("Importing");
    let mut my_vec = Vec::new();
    match coder::importer::import_code(String::from("resources/code"), my_vec.borrow_mut()) {
        Ok(_) => println!("{:?}", my_vec),
        _ => println!("Cannot read/load file")
    }

    let eff = coder::importer::vec2hashmap(my_vec);
    println!("{:?}", eff);

    let eff2= coder::generator::generate_tree(eff);

    println!("{:?}", eff2);
}
