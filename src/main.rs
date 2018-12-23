extern crate structopt;

use exitfailure::ExitFailure;

use s0rt::algorithms::stalin;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Algorithm
    #[structopt(short = "a", long = "algorithm")]
    algorithm: String,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    let sort = match opt.algorithm.as_ref() {
        "stalin" => stalin::sort,
        _ => stalin::sort
    };

    for file in opt.files.iter() {
        let strings: String = std::fs::read_to_string(file)?;
        let lines: Vec<String> = strings.lines().map(String::from).collect();
        println!("{}", sort(&lines).join("\n"))
    }
    Ok(())
}
