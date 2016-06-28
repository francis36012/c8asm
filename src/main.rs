#[macro_use]
extern crate clap;

use std::path::Path;
use std::fs::File;

use clap::{Arg, App};

fn main() {
    let matches = App::new("c8asm")
        .version(crate_version!())
        .author("Francis A. <francisagyapong2@gmail.com>")
        .about("A Chip-8 assembler")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .value_name("FILE")
             .help("The text file (assembly) to be assembled")
             .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("File name of the assembled output")
            .required(true))
        .get_matches();

    let input_file_path = Path::new(matches.value_of("input").unwrap());
    let output_file_path = Path::new(matches.value_of("output").unwrap());

    let input_file = File::open(input_file_path).unwrap();
    let output_file = File::open(output_file_path).unwrap();
}
