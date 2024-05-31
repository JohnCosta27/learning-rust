use std::{fs::File, io::BufReader};
use std::io::BufRead;

use regex::Regex;

fn str_number_to_number(s: &str) -> u32 {
    if s.len() == 1 {
        return s.parse().unwrap();
    }

    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("i dont exist"),
    }
}

fn main() {
    let file = File::open("./src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let number_regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]{1}").unwrap();

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        {
            let mut first_number: u32 = 0;
            for c in line.chars() {
                if c >= '0' && c <= '9' {
                    first_number = c.to_digit(10).unwrap();
                    break;
                }
            } 

            let mut second_number: u32 = 0;
            for c in line.chars().rev() {
                if c >= '0' && c <= '9' {
                    second_number = c.to_digit(10).unwrap();
                    break;
                }
            } 

            part1 += first_number * 10 + second_number;
        }

        {
            let capture = number_regex.captures(&line).unwrap();
            let (matched_value, []) = capture.extract();
            let first_number = str_number_to_number(matched_value);

            let mut copied_line = line.as_str();
            let mut second_number: u32 = 0;

            loop {
                let capture = match number_regex.captures(copied_line) {
                    Some(v) => v,
                    None => break,
                };

                let my_match = capture.get(0).unwrap();
                second_number = str_number_to_number(my_match.as_str());

                let match_index = my_match.start();
                copied_line = &copied_line[match_index+1..];
            }

            part2 += first_number * 10 + second_number;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
