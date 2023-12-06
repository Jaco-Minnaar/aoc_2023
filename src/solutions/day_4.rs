use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day4_1;

impl Solution for Day4_1 {
    fn solve(&self, data: &str) -> anyhow::Result<String> {
        //         let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let total = data
            .trim()
            .split('\n')
            .map(|line| {
                if let [card, nums] = line.trim().split(':').collect::<Vec<&str>>().as_slice() {
                    let card_num = card.split(' ').last().unwrap();
                    let mut num_parts = nums.split('|');
                    let winning_nums = num_parts
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>();
                    let guessed_nums = num_parts
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>();

                    let matches = winning_nums.intersection(&guessed_nums).count();

                    let points = if matches == 0 {
                        0
                    } else {
                        2u32.pow((matches - 1) as u32)
                    };

                    points
                } else {
                    unreachable!()
                }
            })
            .fold(0, |total, points| total + points);

        Ok(total.to_string())
    }
}

pub struct Day4_2;

impl Solution for Day4_2 {
    fn solve(&self, data: &str) -> anyhow::Result<String> {
        //         let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut card_amounts = data.trim().split('\n').map(|_| 1).collect::<Vec<u32>>();

        let lines = data.trim().split('\n');

        for (idx, line) in lines.enumerate() {
            let mut num_parts = line.trim().split(':').last().unwrap().split('|');

            let winning_nums = num_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let guessed_nums = num_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let matches = winning_nums.intersection(&guessed_nums).count();

            for _ in 0..card_amounts[idx] {
                for idx in idx + 1..=idx + matches {
                    card_amounts[idx] += 1;
                }
            }
        }

        let total: u32 = card_amounts.iter().sum();

        Ok(total.to_string())
    }
}
