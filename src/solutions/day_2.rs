use anyhow::Result;

use crate::solution::Solution;

pub struct Day2_1;

impl Solution for Day2_1 {
    fn solve(&self, data: &str) -> Result<String> {
        //         let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines = data.trim().split('\n');
        let mut sum = 0;

        for line in lines {
            let mut split = line.split(':');
            let game = split
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()?;
            let reveals = split.next().unwrap().trim().split(';');
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            for reveal in reveals {
                let splits = reveal.trim().split(',').map(|val| val.trim().split(' '));

                for mut split in splits {
                    let num = split.next().unwrap().parse::<u32>()?;
                    let color = split.next().unwrap();

                    match color {
                        "blue" => {
                            if num > blue {
                                blue = num;
                            }
                        }
                        "red" => {
                            if num > red {
                                red = num;
                            }
                        }
                        "green" => {
                            if num > green {
                                green = num;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }

            if blue <= 14 && red <= 12 && green <= 13 {
                sum += game;
            }
        }

        Ok(sum.to_string())
    }
}

pub struct Day2_2;

impl Solution for Day2_2 {
    fn solve(&self, data: &str) -> Result<String> {
        // let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines = data.trim().split('\n');
        let mut sum = 0;

        for line in lines {
            let mut split = line.split(':');
            let game = split
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()?;
            let reveals = split.next().unwrap().trim().split(';');
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            for reveal in reveals {
                let splits = reveal.trim().split(',').map(|val| val.trim().split(' '));

                for mut split in splits {
                    let num = split.next().unwrap().parse::<u32>()?;
                    let color = split.next().unwrap();

                    match color {
                        "blue" => {
                            if num > blue {
                                blue = num;
                            }
                        }
                        "red" => {
                            if num > red {
                                red = num;
                            }
                        }
                        "green" => {
                            if num > green {
                                green = num;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }

            let power = red * green * blue;
            sum += power;
        }

        Ok(sum.to_string())
    }
}
