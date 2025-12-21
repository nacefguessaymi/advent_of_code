use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_selection = args.get(1).map(String::as_str);
    match day_selection {
        Some("01") => {
            if let Err(e) = day1::main() {
                eprintln!("Error running Day 01: {}", e);
                std::process::exit(1);
            }
        }
        Some("02") => {
            if let Err(e) = day2::main() {
                eprintln!("Error running Day 02: {}", e);
                std::process::exit(1);
            }
        }
        Some("03") => {
            if let Err(e) = day3::main() {
                eprintln!("Error running Day 03: {}", e);
                std::process::exit(1);
            }
        }
        Some("04") => {
            if let Err(e) = day4::main() {
                eprintln!("Error running Day 04: {}", e);
                std::process::exit(1);
            }
        }
        Some("05") => {
            if let Err(e) = day5::main() {
                eprintln!("Error running Day 05: {}", e);
                std::process::exit(1);
            }
        }
        Some("06") => {
            if let Err(e) = day6::main() {
                eprintln!("Error running Day 06: {}", e);
                std::process::exit(1);
            }
        }
        Some("07") => {
            if let Err(e) = day7::main() {
                eprintln!("Error running Day 07: {}", e);
                std::process::exit(1);
            }
        }
        Some("08") => {
            if let Err(e) = day8::main() {
                eprintln!("Error running Day 08: {}", e);
                std::process::exit(1);
            }
        }
        Some("09") => {
            if let Err(e) = day9::main() {
                eprintln!("Error running Day 09: {}", e);
                std::process::exit(1);
            }
        }
        Some("10") => {
            if let Err(e) = day10::main() {
                eprintln!("Error running Day 10: {}", e);
                std::process::exit(1);
            }
        }
        Some("11") => {
            if let Err(e) = day11::main() {
                eprintln!("Error running Day 11: {}", e);
                std::process::exit(1);
            }
        }
        Some("12") => {
            if let Err(e) = day12::main() {
                eprintln!("Error running Day 12: {}", e);
                std::process::exit(1);
            }
        }
        Some(_) => {
            println!("Invalid day!");
        }
        _ => {
            println!("No day specified!");
        }
    }
}
