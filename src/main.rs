/*
 * Copyright (c) 2022. Calloway Sutton
 */

use std::ops::Deref;
use arraylist::arl::ArrayList;
#[allow(dead_code)]

use crate::binary_tree::BTree;
use rand::{thread_rng, Rng};
use bitlab::*;
use crate::encrypt::{encrypt8byte, format_data, gen_rand_key, llkey2arr, print_message};

mod binary_tree;
mod encrypt;

fn read_file(path: String) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(data) => { data }
        Err(_) => { panic!("Could not access the file...") }
    }
}

fn main() {
    let mut bt: BTree<u8> = BTree::new();
    let mut rng = thread_rng();

    // Generate a key
    while bt.get_size() < 64 {
        bt.insert(rng.gen_range(0..64));
    }

    // Make an array of the key
    let mut key = gen_rand_key(bt);

    println!("Key Used:");
    println!("{:?}\n", key);

    let data = read_file(String::from("src/texts/encryptme.txt"));
    // let data: Vec<u8> = String::from("shenzhen").into_bytes();
    let message = format_data(data);

    print_message(message);

    println!("\nMessage: ");
    println!("{:b}", message);
    println!("{:?}", message.to_be_bytes());
    println!("Encrypted: ");
    let encrypted = encrypt8byte(message, key);
    println!("{:b}", encrypted);
    println!("{:?}",encrypted.to_be_bytes());

}
