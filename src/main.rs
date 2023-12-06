mod data;
mod solution;
mod solutions;

use anyhow::{anyhow, bail, Result};
use solutions::{Day1_1, Day1_2, Day2_1, Day2_2, Day3_1, Day3_2, Day4_1, Day4_2, Solutions};
use std::env::args;

fn main() -> Result<()> {
    let solutions = solutions();

    let args = args().collect::<Vec<String>>();

    if args.len() <= 2 {
        bail!("An argument for the which puzzle to do was not passed");
    }

    dbg!(&args);
    let day = args[1].parse::<u32>()?;
    let challenge = args[2].parse::<u32>()?;

    let data = data::fetch_data(day)?;

    let key = format!("{day}.{challenge}");
    let solution = solutions
        .get_solution(key.as_str())
        .ok_or(anyhow!("No matching solution for {key}"))?
        .solve(&data)?;

    println!("{solution}");

    Ok(())
}

fn solutions() -> Solutions<'static> {
    Solutions::new()
        .solution("1.1", Day1_1)
        .solution("1.2", Day1_2)
        .solution("2.1", Day2_1)
        .solution("2.2", Day2_2)
        .solution("3.1", Day3_1)
        .solution("3.2", Day3_2)
        .solution("4.1", Day4_1)
        .solution("4.2", Day4_2)
}
