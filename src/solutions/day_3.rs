use crate::solution::Solution;

#[derive(Debug)]
struct Position {
    line: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Number {
    value: u32,
    position: Position,
}

pub struct Day3_1;

impl Solution for Day3_1 {
    fn solve(&self, data: &str) -> anyhow::Result<String> {
        //         let data = "
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..";

        let mut numbers = vec![];
        let mut symbols = vec![];

        let lines = data.trim().split('\n');

        for (line_num, line) in lines.enumerate() {
            let mut chars = line.chars().enumerate().peekable();
            while let Some((idx, c)) = chars.next() {
                match c {
                    '.' => continue,
                    c if c.is_digit(10) => {
                        let mut num_str = c.to_string();
                        let start = idx;
                        let mut end = idx;
                        while let Some((idx, c)) = chars.peek() {
                            if !c.is_digit(10) {
                                break;
                            }

                            end = *idx;
                            num_str.push(*c);
                            chars.next();
                        }
                        let num = num_str.parse::<u32>()?;

                        numbers.push(Number {
                            value: num,
                            position: Position {
                                line: line_num,
                                start,
                                end,
                            },
                        })
                    }
                    _ => symbols.push(Position {
                        line: line_num,
                        start: idx,
                        end: idx,
                    }),
                }
            }
        }

        let mut sum = 0;
        for number in numbers {
            if symbols
                .iter()
                .filter(|symbol| symbol.line.abs_diff(number.position.line) <= 1)
                .any(|symbol| {
                    (symbol.start >= number.position.start.saturating_sub(1))
                        && symbol.start <= number.position.end + 1
                })
            {
                sum += number.value;
            }
        }

        Ok(sum.to_string())
    }
}

pub struct Day3_2;

impl Solution for Day3_2 {
    fn solve(&self, data: &str) -> anyhow::Result<String> {
        //         let data = "467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..";

        let mut numbers = vec![];
        let mut gears = vec![];

        let lines = data.trim().split('\n');

        for (line_num, line) in lines.enumerate() {
            let mut chars = line.chars().enumerate().peekable();
            while let Some((idx, c)) = chars.next() {
                match c {
                    c if c.is_digit(10) => {
                        let mut num_str = c.to_string();
                        let start = idx;
                        let mut end = idx;
                        while let Some((idx, c)) = chars.peek() {
                            if !c.is_digit(10) {
                                break;
                            }

                            end = *idx;
                            num_str.push(*c);
                            chars.next();
                        }
                        let num = num_str.parse::<u32>()?;

                        numbers.push(Number {
                            value: num,
                            position: Position {
                                line: line_num,
                                start,
                                end,
                            },
                        })
                    }
                    '*' => gears.push(Position {
                        line: line_num,
                        start: idx,
                        end: idx,
                    }),
                    _ => continue,
                }
            }
        }

        dbg!(&numbers[0]);
        dbg!(&gears[0]);
        let mut sum = 0;

        for gear in gears {
            let adjacent: Vec<&Number> = numbers
                .iter()
                .filter(|number| number.position.line.abs_diff(gear.line) <= 1)
                .filter(|number| {
                    (gear.start >= number.position.start.saturating_sub(1))
                        && gear.start <= number.position.end + 1
                })
                .collect();

            dbg!(&adjacent);

            if adjacent.len() == 2 {
                sum += adjacent[0].value * adjacent[1].value;
            }
        }

        Ok(sum.to_string())
    }
}
