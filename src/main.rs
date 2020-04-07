mod dice;
use dice::Dice;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "DnDice", author, about)]
struct Opt {
    /// The set of dice you want to roll, examples:
    ///
    /// "25d2"
    /// "3d20 3d6 d100"
    /// "d10"
    #[structopt(name = "DICE", parse(try_from_str = Dice::new))]
    dice: Vec<Dice>,

    /// Output file, defaults to stdout
    #[structopt(short, long, parse(from_os_str))]
    output: Option<std::path::PathBuf>,

    /// Sum the results into one value
    #[structopt(short, long)]
    sum: bool,

    /// Terse (result-only) output, useful for scripting
    #[structopt(short, long)]
    terse: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let rolls: Vec<_> = opt.dice.iter().map(|x| x.roll()).collect();

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
        let formatted: Vec<_> = if opt.terse {
            rolls.iter().map(|x| x.total().to_string()).collect()
        } else {
            rolls.iter().map(|x| x.pretty()).collect()
        };
        formatted.join("\n")
    };

    // Output to file if configured
    match opt.output {
        Some(path) => std::fs::write(path, format!("{}\n", result))?,
        None => println!("{}", result),
    }

    Ok(())
}
