use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: u32 = 0;
    if let Ok(file) = read_lines("./day1.txt") {
        for lines in file {
            if let Ok(line) = lines {
                let value = parse_digits(&line);
                result += value;
            }
        }
    }
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_digits(s: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();
    for ch in s.chars() {
        if ch.is_digit(10) {
            if let Some(digit) = ch.to_digit(10) {
                digits.push(digit);
            }
        }
    }
    let first = digits[0];
    let last = digits.last().copied();
    let result = (10 * first) + last.unwrap();
    result
}
