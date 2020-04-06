use anyhow::{Context, Result};
use rand::prelude::*;

const NUMBER_ERROR: &'static str = " could not be parsed as a number";

#[derive(Debug)]
pub struct Dice {
    desc: String,
    number: u32,
    value: u32,
}

impl Dice {
    pub fn from_string(string_desc: &str) -> Result<Self> {
        let split: Vec<&str> = string_desc.split('d').collect();
        let value = split[1]
            .parse::<u32>()
            .with_context(|| format!("'{}' {}", split[1], NUMBER_ERROR))?;
        if split[0] == "" {
            Ok(Dice {
                desc: String::from(string_desc),
                number: 1,
                value,
            })
        } else {
            let number = split[0]
                .parse::<u32>()
                .with_context(|| format!("'{}' {}", split[0], NUMBER_ERROR))?;

            Ok(Dice {
                desc: String::from(string_desc),
                number,
                value,
            })
        }
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn vec_from_string(string_desc: String, delim: char) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        for dice in string_desc.split(delim) {
            result.push(Dice::from_string(dice)?);
        }
        Ok(result)
    }

    pub fn roll(&self) -> Vec<u32> {
        let mut rng = thread_rng();
        (0..(self.number))
            .map(|_| rng.gen_range(1, self.value + 1))
            .collect()
    }

    pub fn pretty_roll(rolls: &[u32]) -> String {
        let pretty_rolls: Vec<String> = rolls.iter().map(|x| x.to_string()).collect();
        pretty_rolls.join(" + ")
    }
}
