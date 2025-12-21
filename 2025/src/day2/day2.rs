use std::fs;
use std::error::Error;
use std::ops::Range;
fn check_id(num: u64, answer: &mut u64, answer2: &mut u64) -> u64{
    let str_num: Vec<_> = num.to_string().chars().collect();
    if str_num.len()%2 == 0 {
        let mid_point:usize = (str_num.len()/2).try_into().unwrap();
        if str_num[0..mid_point] == str_num[mid_point..str_num.len()] {*answer += num}
    } else {}
    return *answer;
}

fn range_id(id: &str) -> Range<u64> {
    let id_min_max: Vec<_> = id.split("-").filter_map( |x| x.trim().parse::<u64>().ok()).collect();
    let id_range = Range {start: id_min_max[0], end: id_min_max[1] + 1};
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
        for num in range {check_id(num, &mut answer, &mut answer2);}
    };
    println!("The sum of all invalid IDs is {}", answer);
    Ok(())
}
