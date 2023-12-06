use std::collections::HashSet;

use anyhow::Result;

use crate::solution::Solution;

pub struct Day1_1;

impl Solution for Day1_1 {
    fn solve(&self, data: &str) -> Result<String> {
        let lines = data.split('\n');
        let mut sum = 0;

        for line in lines {
            let chars = line.chars();
            let mut first = None;
            let mut last = None;

            for char in chars {
                if char.is_digit(10) {
                    if first.is_none() {
                        first.replace(char);
                    }

                    last.replace(char);
                }
            }

            if first.is_none() {
                continue;
            }
            let first = first.unwrap();
            let last = last.unwrap();

            let num = format!("{first}{last}").parse::<u32>()?;
            sum += num;
        }

        Ok(sum.to_string())
    }
}

pub struct Day1_2;

impl Solution for Day1_2 {
    fn solve(&self, data: &str) -> Result<String> {
        //         let data = r#"two1nine
        // eightwothree
        // abcone2threexyz
        // xtwone3four
        // 4nineeightseven2
        // zoneight234
        // 7pqrstsixteen"#;
        // let data = "1eightwo\nthree";

        let nums = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digit_chars = nums.join("").chars().collect::<HashSet<char>>();
        let start_chars = nums
            .iter()
            .map(|num| num.chars().next().unwrap())
            .collect::<HashSet<char>>();
        let lines = data.split('\n');
        let mut sum = 0;

        let mut possible_nums: Vec<String> = vec![];
        for line in lines {
            let chars = line.chars();
            let mut first = None;
            let mut last = None;

            for char in chars {
                let digit = if char.is_digit(10) {
                    possible_nums.clear();
                    Some(char)
                } else if digit_chars.contains(&char) {
                    possible_nums.iter_mut().for_each(|num| {
                        num.push(char);
                    });
                    if start_chars.contains(&char) {
                        possible_nums.push(char.to_string());
                    }

                    if let Some(num) = possible_nums
                        .iter()
                        .filter(|num| nums.contains(&num.as_str()))
                        .last()
                    {
                        digit(num)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(digit) = digit {
                    if first.is_none() {
                        first.replace(digit);
                    }

                    last.replace(digit);
                }
            }

            if first.is_none() {
                continue;
            }
            let first = first.unwrap();
            let last = last.unwrap();

            let num = format!("{first}{last}").parse::<u32>()?;
            sum += num;

            println!("{} -> {}", line, num);
            possible_nums.clear();
        }

        Ok(sum.to_string())
    }
}

fn digit(word: &str) -> Option<char> {
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}
