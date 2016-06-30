#[macro_use]
extern crate clap;
extern crate c8asm;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use clap::{Arg, App};

use c8asm::parser::Stream;

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

    let mut stream = Stream::new(input_file.bytes());
    let mut tokens = vec![];

    loop {
        let maybe_token = stream.next_token();
        if maybe_token.is_none() {
            break;
        }
        let token = maybe_token.unwrap();
        tokens.push(token);
    }

    let code = self::c8asm::parser::code_gen(&tokens);
    match code {
        Ok(c) => {
            let mut output_file = File::create(output_file_path).unwrap();
            for i in c {
                let buf = [((i & 0xff00) >> 8) as u8, (i & 0x00ff) as u8];
                output_file.write(&buf);
            }
            std::process::exit(0);
        },
        Err(t) => {
            println!("error: unexpected token: {:?}", t);
            std::process::exit(1);
        }
    }
}
