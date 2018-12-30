extern crate structopt;

use std::io::{self, Write};
use exitfailure::ExitFailure;

use s0rt::algorithms::{get_compare_fn, stalin, bogo, sleep};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Sorting algorithm
    #[structopt(short = "a", long = "algorithm", default_value="stalin")]
    algorithm: String,

    /// Write result to file instead of standard output
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Compare according to general numerical value
    #[structopt(short = "g", long="general_numeric_sort")]
    general_numeric_sort: bool,

    /// Reverse the result of comparison
    #[structopt(short = "r", long="reverse")]
    reverse: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}


fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    let sort = match opt.algorithm.as_str() {
        "stalin" => stalin::sort,
        "bogo" => bogo::sort,
        "sleep" => sleep::sort,
        _ => stalin::sort
    };

    let stdout = io::stdout();
    let mut handle: io::BufWriter<Box<dyn Write>> = match opt.output {
        Some(path) => {
            let file = std::fs::File::open(path)?;
            io::BufWriter::new(Box::new(file))
        }
        _ => {
            let locked_stdout = stdout.lock();
            io::BufWriter::new(Box::new(locked_stdout))
        }
    };

    let cmp = get_compare_fn(opt.reverse);
    for file in opt.files.iter() {
        let strings: String = std::fs::read_to_string(file)?;
        let lines: Vec<String> = strings.lines().map(String::from).collect();

        for line in sort(&lines, &cmp) {
            writeln!(handle, "{}", line)?;
        }
    }
    Ok(())
}
