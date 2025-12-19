#!/usr/bin/env rustx
use std::fs;
use std::error::Error;
fn lock_sim(dial: &i64, num: i64, clicks: &mut u64, num_zer: &mut u64) -> (i64, u64) {
    let mut alr_zero: bool = false;
    if *dial == 0 {alr_zero = true};
    let summ: i64 = dial + num;
    let curr_click = (summ/100).abs() as u64;
    let dial = summ.rem_euclid(100);
    println!("The dial is rotated by {} to point at {}", num, dial);
    if summ < 0 && curr_click == 0 && alr_zero == false && dial != 0 {
        *clicks += 1;
        println!("During this rotation the dial pointed to 0 once.")
    } else if curr_click > 0 && dial != 0 {
        *clicks += curr_click;
        println!("During this rotation it points at zero {} times.", curr_click);
    } else if curr_click > 0 && dial == 0 {
        *clicks += curr_click - 1
    };
    if dial == 0 {
        *num_zer += 1;
        println!("After this rotation is done the dial is at zero.")
    };
    return (dial, *clicks);
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("src/day1/puzzle.txt")?;
    let mut current_dial: i64 = 50;
    let mut num_zer: u64 = 0;
    let mut clicks: u64 = 0;
    println!("The dial starts by pointing {}", current_dial);
    for rotation in file.lines() {
        let direction: char = rotation.chars().next().unwrap();
        let direction: i16 = match direction { 'R' => 1, 'L' => -1, _ => todo!()};
        let num = direction * rotation[1..].parse::<i16>()?;
        (current_dial, clicks) = lock_sim(&current_dial, num.into(), &mut clicks, &mut num_zer);
    };
    println!("Following the security training seminar the number of zeros was: {}.\nFollowing the 0x434C49434B convention the number of zeros is: {}.",num_zer, num_zer + clicks);
    Ok(())
}
