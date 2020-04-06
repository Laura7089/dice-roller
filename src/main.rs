mod dice;
use dice::Dice;
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

    /// Delimiting character for the list of dice, default ","
    #[structopt(short, long)]
    delimiter: Option<char>,

    /// Sum the results into one value
    #[structopt(short, long)]
    sum: bool,

    /// Terse (result-only) output
    #[structopt(short, long)]
    terse: bool,
    // /// CSV output
    // #[structopt(short, long)]
    // csv: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let dice = Dice::vec_from_string(opt.dice, opt.delimiter.unwrap_or(','))?;
    let rolls: Vec<Vec<u32>> = dice.iter().map(|x| x.roll()).collect();

    let result = if opt.sum {
        let sum = rolls.iter().map(|x| x.iter().sum::<u32>()).sum::<u32>();
        if opt.terse {
            sum.to_string()
        } else {
            let pretty = rolls
                .iter()
                .map(|x| Dice::pretty_roll(x))
                .collect::<Vec<String>>()
                .join(")\n+ (");
            format!("{}\n= ({})", sum, pretty)
        }
    } else {
        if opt.terse {
            rolls
                .iter()
                .map(|x| x.iter().sum::<u32>().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            rolls
                .iter()
                .enumerate()
                .map(|(num, x)| {
                    format!(
                        "{}: {} = {}",
                        dice[num].desc(),
                        x.iter().sum::<u32>(),
                        Dice::pretty_roll(x)
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
    };

    match opt.output {
        Some(path) => fs::write(path, format!("{}\n", result))?,
        None => println!("{}", result),
    }

    Ok(())
}
