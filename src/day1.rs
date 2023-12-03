#![allow(dead_code)]
use std::io::Read;

pub fn run() {
    let mut file = std::fs::File::open("day1/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total: u32 = 0;

    for line in contents.split('\n') {
        let mut line_parsed = line.to_string();

        for (to_replace, value) in [
            ("oneight", "18"),
            ("threeight", "38"),
            ("fiveeight", "58"),
            ("sevenine", "79"),
            ("eightwo", "82"),
            ("twone", "21"),
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9")
        ] {
            line_parsed = line_parsed.replace(to_replace, value);
        }

        let mut found_digits = 0;
        let mut current_index = 0;
        let mut first_digit: u32 = 0;
        for (index, char) in line_parsed.chars().enumerate() {
            if char.is_ascii_digit() {
                if found_digits == 0 {
                    first_digit = char.to_digit(10).unwrap();
                }
                current_index = index;
                found_digits += 1;
            }
        }
        if found_digits == 0 {
            continue;
        }
        let last_digit: u32 = line_parsed
            .chars()
            .nth(current_index)
            .unwrap()
            .to_digit(10)
            .unwrap();
        total += first_digit * 10 + last_digit;
    }

    println!("{}", total);
}
