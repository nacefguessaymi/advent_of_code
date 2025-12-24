use std::error::Error;
use std::fs;
pub fn main() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("src/day6/puzzle.txt")?;
    let lines = file.lines();
    let operations: Vec<_> = lines.last().unwrap().split_whitespace().collect();
    println!("{:?}", operations);
    let mut answer = 0;
    Ok(())
}
