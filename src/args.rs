use std::path::Path;

use clap::Parser;

#[derive(Parser)]
pub struct Arg {
    #[clap(value_name = "FILE")]
    pub entry_point: String, //arguement for command line app
}
