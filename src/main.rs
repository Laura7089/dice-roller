mod dice;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Dice Roller", author, about)]
struct Opt {
    /// The delimited set of dice you want to roll, eg "3d20,3d6,d100"
    #[structopt(name = "DICE")]
    dice: String,

    /// Output file, leave blank for stdout
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// The delimiting character for the list of dice
    #[structopt(short, long, default_value = ",")]
    delimiter: char,

    /// Sum the results into one value
    #[structopt(short, long)]
    sum: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let dice = dice::Dice::vec_from_string(opt.dice, opt.delimiter)?;

    let result = if opt.sum {
        format!("{}", dice.iter().map(|x| x.total_roll()).sum::<u32>())
    } else {
        format!(
            "{}",
            dice.iter()
                .map(|x| x.pretty_roll())
                .collect::<Vec<String>>()
                .join("\n")
        )
    };

    match opt.output {
        Some(path) => fs::write(path, format!("{}\n", result))?,
        None => println!("{}", result),
    }

    Ok(())
}
