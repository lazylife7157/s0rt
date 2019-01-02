extern crate structopt;

use std::io::{self, Write};
use exitfailure::ExitFailure;
use structopt::StructOpt;

use s0rt::Opt;
use s0rt::{
    stalin_sort,
    bogo_sort,
    sleep_sort
};

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    let stdout = io::stdout();
    let mut handle: io::BufWriter<Box<dyn Write>> = io::BufWriter::new(match opt.output {
        Some(path) => Box::new(std::fs::File::open(path)?),
        None => Box::new(stdout.lock())
    });

    for file in opt.files.iter() {
        let strings: String = std::fs::read_to_string(file)?;
        let lines: Vec<String> = strings.lines().map(String::from).collect();
        let sorted = match opt.algorithm.as_str() {
            "stalin" => stalin_sort(&lines, opt.reverse, opt.general_numeric_sort),
            "bogo" => bogo_sort(&lines, opt.reverse, opt.general_numeric_sort),
            "sleep" => sleep_sort(&lines),
            _ => stalin_sort(&lines, opt.reverse, opt.general_numeric_sort),
        };

        for line in sorted {
            writeln!(handle, "{}", line)?;
        }
    }
    Ok(())
}
