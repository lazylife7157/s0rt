extern crate structopt;

pub use self::algorithms::stalin_sort;
pub use self::algorithms::bogo_sort;
pub use self::algorithms::sleep_sort;

use std::path::PathBuf;
use structopt::StructOpt;

mod algorithms;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    /// Sorting algorithm
    #[structopt(short = "a", long = "algorithm", default_value="stalin")]
    pub algorithm: String,

    /// Write result to file instead of standard output
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Compare according to general numerical value
    #[structopt(short = "g", long="general_numeric_sort")]
    pub general_numeric_sort: bool,

    /// Reverse the result of comparison
    #[structopt(short = "r", long="reverse")]
    pub reverse: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
