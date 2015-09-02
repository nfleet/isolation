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
use std::io::{BufReader, BufWriter, Write, Read, stdout, stdin, stderr};
use getopts::Options;

static USAGE: &'static str = "Usage: isol [options] <SOURCE> <<TARGET>>";

fn main() {
    let mut opts = Options::new();
    opts.optflag("a", "all", "include the nodes of the largest component (prints all nodes of the graph)");
    opts.optflag("h", "help", "show this help");
    let argv: Vec<String> = env::args().collect();
    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage(USAGE));
        process::exit(1);
    }

    let print_biggest = matches.opt_present("a");
    let args = matches.free;

    let (input, output): (Box<Read>, Box<Write>) = match args.len() {
        2 => {
            let i = File::open(&args[0]).unwrap();
            let f = File::create(&args[1]).unwrap();
            (Box::new(i), Box::new(f))
        },
        1 => {
            let i = File::open(&args[0]).unwrap();
            (Box::new(i), Box::new(stdout()))
        },
        0 => {
            (Box::new(stdin()), Box::new(stdout()))
        },
        _ => {
            writeln!(&mut stderr(), "error: too many arguments provided").unwrap();
            println!("{}", opts.usage(USAGE));
            process::exit(1);
        }
    };

    let mut source = BufReader::new(input);
    let mut target = BufWriter::new(output);
    compute(&mut source, |comp, i| {
        if !print_biggest && i == 0 {
            return;
        }
        serialize(comp, &mut target, b'\n');
    });
}
