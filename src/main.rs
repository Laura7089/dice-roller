mod dice;
use dice::Dice;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Dice Roller", author, about)]
struct Opt {
    /// The delimited set of dice you want to roll, eg "3d20,3d6,d100"
    #[structopt(name = "DICE")]
    dice: String,

    /// Output file, leave blank for stdout
    #[structopt(short, long, parse(from_os_str))]
    output: Option<std::path::PathBuf>,

    /// Delimiting character for the list of dice, default ","
    #[structopt(short, long)]
    delimiter: Option<char>,

    /// Sum the results into one value
    #[structopt(short, long)]
    sum: bool,

    /// Terse (result-only) output
    #[structopt(short, long)]
    terse: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let delimiter = opt.delimiter.unwrap_or(',');

    // Roll our dice
    let rolls: Vec<_> = Dice::new_vec_roll(opt.dice, delimiter)?;

    // Format the results
    let result = if opt.sum {
        // Sum
        let sum: u32 = rolls.iter().map(|x| x.total()).sum();
        if opt.terse {
            sum.to_string()
        } else {
            let pretty: Vec<_> = rolls.iter().map(|x| x.pretty_bare()).collect();
            format!("{}\n= ({})", sum, pretty.join(")\n+ ("))
        }
    } else {
        // Regular
        let formatted: Vec<_>;
        if opt.terse {
            formatted = rolls.iter().map(|x| x.total().to_string()).collect();
        } else {
            formatted = rolls.iter().map(|x| x.pretty()).collect();
        }
        formatted.join("\n")
    };

    // Output to file if configured
    match opt.output {
        Some(path) => std::fs::write(path, format!("{}\n", result))?,
        None => println!("{}", result),
    }

    Ok(())
}
