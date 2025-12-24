use std::error::Error;
use std::fs;
use std::ops::Range;
fn check_id(num: u64, answer: &mut u64, answer2: &mut u64) -> (u64, u64) {
    let str_num: Vec<_> = num.to_string().chars().collect();
    if str_num.len() % 2 == 0 {
        let mid_point: usize = (str_num.len() / 2).try_into().unwrap();
        if str_num[0..mid_point] == str_num[mid_point..str_num.len()] {
            *answer += num
        }
    };
    let mut counted = false;
    // Iterates through the length of the string so we can get predefined points
    // (ie. for a 10 digit number point would be 1, 2, 3, 4,..., 10)
    for point in 1..str_num.len() {
        if str_num.len() % point != 0 {
            continue;
        };
        // Define out substring so to make syntax easier
        // This will be through the iteration of point
        // So for example 38593859 at point = 4, sub would be
        // sub = 3859
        // We then define boolean rep to be used later
        let sub = &str_num[..point];
        let mut rep = true;
        // We iterate through the remaining length of the string
        // So if point = 4 and num is 38593859 we step through the rest
        // of the string of this number from 3959 and our step will be ranging from 0 to 2
        for step in 0..(str_num.len() / point) {
            // When step = 0 this will be the first 4 chars if point = 4
            // so it will match to itself
            // When step = 1 it will try to match 3859 to 3859
            // If for a number like 38593858 sub will not match itself and rep will become false
            if sub != &str_num[step * point..(step + 1) * point] {
                rep = false;
                break;
            }
        }
        // We use counted to make sure we did not double count
        if rep && counted != true {
            println! {"Dynamic checker found {}", num}
            *answer2 += num;
            counted = true;
        }
    }
    return (*answer, *answer2);
}

fn range_id(id: &str) -> Range<u64> {
    let id_min_max: Vec<_> = id
        .split("-")
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();
    let id_range = Range {
        start: id_min_max[0],
        end: id_min_max[1] + 1,
    };
    return id_range;
}
pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Running day 2 solution...");
    let ids: String = fs::read_to_string("src/day2/puzzle.txt")?;
    let ids = ids.split(",");
    let mut answer: u64 = 0;
    let mut answer2: u64 = 0;
    for id in ids {
        let range = range_id(&id);
        for num in range {
            check_id(num, &mut answer, &mut answer2);
        }
    }
    println!("The sum of all invalid IDs is {}", answer);
    println!("The sum of all invalid IDs with ranging is {}", answer2);
    Ok(())
}
