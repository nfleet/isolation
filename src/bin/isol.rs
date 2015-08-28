//! **isol** is a program that uses the [**isolation**](../isolation) library
//! to compute graph components. It takes two parameters: the input file which is the graph,
//! and the output file where the results are stored. See the [**compute**](../isolation/isolation/fn.compute.html)
//! for the graph format. If target is omitted, it will output to stdout. If input is omitted,
//! it will read from stdin.

extern crate isolation;
extern crate getopts;

use std::process;
use isolation::isolation::{compute, serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, stdout, stdin};
use getopts::Options;

fn usage(opts: Options) {
    let brief = format!("Usage: isol [options] <SOURCE> <<TARGET>>");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("a", "all", "print all nodes (including ones of the largest component)");
    opts.optflag("h", "help", "show this help");
    let argv: Vec<String> = env::args().collect();
    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        usage(opts);
        process::exit(1);
    }

    let print_biggest = matches.opt_present("a");
    let args = matches.free;

    match args.len() {
        2 => {
            let i = File::open(&args[0]).unwrap();
            let f = File::create(&args[1]).unwrap();
            let mut input = BufReader::new(i);
            let mut file = BufWriter::new(&f);
            let comps = compute(&mut input);
            serialize(comps, &mut file, b'\n', print_biggest);
        },
        1 => {
            let i = File::open(&args[0]).unwrap();
            let mut input = BufReader::new(i);
            let comps = compute(&mut input);
            serialize(comps, &mut BufWriter::new(stdout()), b'\n', print_biggest);
        },
        0 => {
            let comps = compute(&mut BufReader::new(stdin()));
            serialize(comps, &mut BufWriter::new(stdout()), b'\n', print_biggest);
        },
        _ => {
            usage(opts);
            process::exit(1);
        }
    }

}
