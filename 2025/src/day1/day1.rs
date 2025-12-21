use std::fs;
use std::error::Error;
fn lock_sim(dial: &i64, num: i64, clicks: &mut u64, num_zer: &mut u64) -> (i64, u64) {
    println!("Adding {} to dial at {}", num, dial);
    let mut curr_dial: i64 = *dial;
    let dial: i64 = dial + num;
    println!("Dial is now at {}", dial);
    let mut i = 1;
    while i <= num.abs() {
        if num < 0 {            
            curr_dial -= 1;
        } else if num > 0 {
            curr_dial += 1;
        };
        if curr_dial%100 == 0 {
            *clicks +=1;
        };
        i += 1;
    };
    if dial%100 == 0 {
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
    println!("Following the security training seminar the number of zeros was: {}.\nFollowing the 0x434C49434B convention the number of zeros is: {}.",num_zer, clicks);
    Ok(())
}
