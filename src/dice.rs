use rand::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DiceError;

impl Error for DiceError {}

impl Display for DiceError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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
    pub fn new(string_desc: &str) -> Result<Self, DiceError> {
        let split: Vec<_> = string_desc.split('d').collect();
        match &split[..] {
            ["", val_raw] => Ok(Dice {
                desc: string_desc.into(),
                number: 1,
                value: val_raw.parse::<u32>().or(Err(DiceError))?,
            }),
            [num_raw, val_raw] => Ok(Dice {
                desc: string_desc.into(),
                number: num_raw.parse::<u32>().or(Err(DiceError))?,
                value: val_raw.parse::<u32>().or(Err(DiceError))?,
            }),
            _ => Err(DiceError),
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
}

impl Roll {
    pub fn pretty_bare(&self) -> String {
        let pretty_rolls: Vec<_> = self.rolls.iter().map(|x| x.to_string()).collect();
        pretty_rolls.join(" + ")
    }

    pub fn pretty(&self) -> String {
        format!("{}: {} = {}", self.desc, self.total(), self.pretty_bare())
    }

    pub fn total(&self) -> u32 {
        self.rolls.iter().sum()
    }
}
