use anyhow::{Context, Result};
use rand::prelude::*;

#[derive(Debug)]
struct DiceError;

impl std::error::Error for DiceError {}

impl std::fmt::Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bad dice formatting.\nShould be [<number>]d<number>, like '10d6' or 'd8'"
        )
    }
}

#[derive(Debug)]
pub struct Dice {
    desc: String,
    number: u32,
    value: u32,
}

impl Dice {
    pub fn from_string(string_desc: &str) -> Result<Self> {
        let split: Vec<&str> = string_desc.split('d').collect();
        split
            .get(1)
            .ok_or(DiceError)
            .with_context(|| string_desc.to_owned())?;
        let value = split[1]
            .parse::<u32>()
            .map_err(|_| DiceError)
            .with_context(|| string_desc.to_owned())?;
        if split[0] == "" {
            Ok(Dice {
                desc: String::from(string_desc),
                number: 1,
                value,
            })
        } else {
            let number = split[0]
                .parse::<u32>()
                .with_context(|| split[0].to_owned())?;

            Ok(Dice {
                desc: string_desc.to_owned(),
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
