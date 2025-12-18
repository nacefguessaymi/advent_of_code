#!/usr/bin/env rustx

use std::fs::File

pub fn open_puzzle() {
    
}

pub fn lock_sim(dial: u8, num: u8, direction: bool) -> u8 {
    let dial: u8 = match direction {
        true => {dial+num%100}
        false => {dial-num%100}
    }
}

fn main() {
    let mut current_dial: u8 = 50
    let mut num_zer: u32 = 0
}
