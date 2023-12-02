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

    let mut str = s.to_string().clone();
    let len = str.len();
    let mut i = 0;
    
    while i < len {
        let ch = str.chars().nth(0).unwrap();
        i += 1;
        if ch.is_digit(10) {
            if let Some(digit) = ch.to_digit(10) {
                digits.push(digit);
            }
        }

        match ch {
            'o' => if str.starts_with("one") {
                digits.push(1);
            },
            't' => if str.starts_with("two") {
                digits.push(2);
            } else if str.starts_with("three") {
                digits.push(3);
            },
            'f' => if str.starts_with("four") {
                digits.push(4);
            } else if str.starts_with("five") {
                digits.push(5);
            },
            's' => if str.starts_with("six") {
                digits.push(6);
            } else if str.starts_with("seven") {
                digits.push(7);
            },
            'e' => if str.starts_with("eight") {
                digits.push(8);
            },
            'n' => if str.starts_with("nine") {
                digits.push(9);
            },
            _ => {

            }
        }

        str.remove(0);
    }
    let first = digits[0];
    let last = digits.last().copied();
    let result = (10 * first) + last.unwrap();
    result
}
