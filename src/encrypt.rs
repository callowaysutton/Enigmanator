/*
 * Copyright (c) 2022. Calloway Sutton
 */

use std::collections::LinkedList;
use bitlab::SingleBits;
use crate::binary_tree::BTree;

pub fn encrypt8byte(message: u64, key: [u8;64]) -> u64 {
    let mut encrypted: u64 = 0;

    for pos in 0..64 {
        if message.get_bit(key[pos] as u32).unwrap() {
            encrypted = encrypted.set_bit(pos as u32).unwrap();
        }
    }

    encrypted
}

pub fn llkey2arr(ll: LinkedList<&u8>) -> [u8;64] {
    let mut key: [u8; 64] = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2];
    let mut i = 0;
    for n in ll {
        key[i] = *n;
        i += 1;
    }
    key
}

pub fn gen_rand_key(bt: BTree<u8>) -> [u8;64] {
    let list = bt.list_postorder().unwrap();
    llkey2arr(list)
}

pub fn format_data(data: Vec<u8>) -> u64 {
    let mut nums = [2,2,2,2,2,2,2,2];
    let mut count = 0;
    for datum in data {
        nums[count] = datum;
        count += 1;
    }

    return as_u64_be(&nums);
}

pub fn print_message(message: u64) {
    for num in message.to_be_bytes() {
        let c = num as char;
        println!("{:08b} => {:} => {:}", num, num, c);
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