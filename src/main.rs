/*
 * Copyright (c) 2022. Calloway Sutton
 */

use std::ops::Deref;
use arraylist::arl::ArrayList;
#[allow(dead_code)]

use crate::binary_tree::BTree;
use rand::{thread_rng, Rng};
use bitlab::*;
mod binary_tree;

fn read_file(path: String) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(data) => { data }
        Err(_) => { panic!("Could not access the file...") }
    }
}

fn as_u64_be(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) << 56) +
        ((array[1] as u64) << 48) +
        ((array[2] as u64) << 40) +
        ((array[3] as u64) <<  32) +
        ((array[4] as u64) << 24) +
        ((array[5] as u64) << 16) +
        ((array[6] as u64) <<  8) +
        ((array[7] as u64) <<  0)
}

fn as_u64_le(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) << 0) +
        ((array[1] as u64) << 8) +
        ((array[2] as u64) << 16) +
        ((array[3] as u64) <<  24) +
        ((array[4] as u64) << 32) +
        ((array[5] as u64) << 40) +
        ((array[6] as u64) <<  48) +
        ((array[7] as u64) <<  56)
}


fn main() {
    let mut bt: BTree<u8> = BTree::new();
    let mut rng = thread_rng();
    let mut nums: [u8;8] = [0,0,0,0,0,0,0,0];
    // let data = read_file(String::from("src/encryptme.txt"));

    // Generate a key
    while bt.get_size() < 64 {
        bt.insert(rng.gen_range(0..64));
    }

    println!("Key Used:");
    bt.print_postorder();
    let list = bt.list_postorder().unwrap();
    println!("\n");

    // Make an array of the key
    let mut key: [u8; 64] = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2];
    let mut i = 0;
    for n in list {
        key[i] = *n;
        i += 1;
    }

    let data: Vec<u8> = String::from("shenzhen").into_bytes();
    let mut count = 0;
    for datum in data {
        nums[count] = datum;
        count += 1;
    }

    for num in nums {
        let c = num as char;
        println!("{:b} => {:} => {:}", num, num, c);
    }

    let a = as_u64_be(&nums);
    let mut b: u64 = 0;

    println!("\nMessage: ");
    println!("{:b}", a);
    println!("Encrypted: ");
    for pos in 0..64 {
        if a.get_bit(key[pos] as u32).unwrap() {
            b = b.set_bit(pos as u32).unwrap();
        }
    }

    println!("{:b}", b);
    // bt.print_preorder();



    // println!("\n{:?}", bt.list_postorder().unwrap());
}
