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
    pub desc: String,
    number: u32,
    value: u32,
}

#[derive(Debug)]
pub struct Roll {
    pub desc: String,
    rolls: Vec<u32>,
}

impl Dice {
    pub fn new(string_desc: &str) -> Result<Self> {
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

    pub fn roll(&self) -> Roll {
        let mut rng = thread_rng();
        let rolls = (0..(self.number))
            .map(|_| rng.gen_range(1, self.value + 1))
            .collect();
        Roll {
            desc: self.desc.clone(),
            rolls,
        }
    }

    pub fn new_vec(string_desc: String, delim: char) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        for dice in string_desc.split(delim) {
            result.push(Dice::new(dice)?);
        }
        Ok(result)
    }

    pub fn roll_all(dice: Vec<Dice>) -> Vec<Roll> {
        dice.into_iter().map(|x| x.roll()).collect()
    }

    pub fn new_vec_roll(string_desc: String, delim: char) -> Result<Vec<Roll>> {
        Ok(Dice::roll_all(Dice::new_vec(string_desc, delim)?))
    }
}

impl Roll {
    pub fn pretty_bare(&self) -> String {
        let pretty_rolls: Vec<String> = self.rolls.iter().map(|x| x.to_string()).collect();
        pretty_rolls.join(" + ")
    }

    pub fn pretty(&self) -> String {
        format!("{}: {} = {}", self.desc, self.total(), self.pretty_bare())
    }

    pub fn total(&self) -> u32 {
        self.rolls.iter().sum()
    }
}
