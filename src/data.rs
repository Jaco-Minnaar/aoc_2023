use std::fs;

use anyhow::{bail, Result};

pub fn fetch_data(day: u32) -> Result<String> {
    let file_path = format!("data/day_{day}.txt");

    let data = if let Ok(bytes) = fs::read(&file_path) {
        String::from_utf8(bytes)?
    } else {
        let link = format!("https://adventofcode.com/2023/day/{}/input", day);

        let client = reqwest::blocking::Client::builder().build()?;

        let response = client.get(link).header("Cookie", "session=53616c7465645f5f034b3ba52e8e5ed62fb83b0c58bebc2c8aca682224a246d66864def9b7b3937295f4538e5ab7c831105fc2d45b688c36d9801dacee917ca7").send()?;

        if !response.status().is_success() {
            bail!("There was an error getting data: {}", response.text()?);
        }

        let data = response.text()?;

        fs::write(&file_path, &data)?;

        data
    };

    Ok(data)
}
