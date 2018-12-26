extern crate structopt;

use std::io::{self, Write};
use exitfailure::ExitFailure;

use s0rt::algorithms::stalin;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Sorting algorithm
    #[structopt(short = "a", long = "algorithm", default_value="stalin")]
    algorithm: String,

    /// Write result to file instead of standard output
    #[structopt(short = "o", long = "output", parse(from_os_str), default_value = "")]
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

    let stdout = io::stdout();
    let mut handle: io::BufWriter<Box<Write>> = match opt.output.to_str() {
        Some(path) if path != "" => {
            let file = std::fs::File::open(path)?;
            io::BufWriter::new(Box::new(file))
        }
        _ => {
            let locked_stdout = stdout.lock();
            io::BufWriter::new(Box::new(locked_stdout))
        }
    };

    for file in opt.files.iter() {
        let strings: String = std::fs::read_to_string(file)?;
        let lines: Vec<String> = strings.lines().map(String::from).collect();

        for line in sort(&lines) {
            writeln!(handle, "{}", line)?;
        }
    }
    Ok(())
}
