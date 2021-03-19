use md5::{Md5, Digest};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let mut rainbow_table = HashMap::new();
    for i in 0..2_i32.pow(30) {
        let string = i.to_string();
        let result = format!("{:x}", Md5::digest(string.as_bytes()));
        let hash: String =  result[result.len()-6..].to_string();

        rainbow_table.entry(hash).or_insert(string);

        if rainbow_table.len() == 16_usize.pow(6) {
            break
        }
    }

    let rainbow_file = File::create("rainbow.cbor").unwrap();

    println!("Size of rainbow table: {}", rainbow_table.len());

    serde_cbor::to_writer(rainbow_file, &rainbow_table);

}
