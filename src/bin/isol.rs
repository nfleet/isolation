//! **isol** is a program that uses the [**isolation**](../isolation) library
//! to compute graph components. It takes two parameters: the input file which is the graph,
//! and the output file where the results are stored. See the [**compute**](../isolation/isolation/fn.compute.html)
//! for the graph format. If target is omitted, it will output to stdout. If input is omitted,
//! it will read from stdin.
extern crate isolation;

use std::process;
use isolation::isolation::{compute, serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, stdout, stderr, stdin, Write};

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        3 => {
            let i = File::open(&args[1]).unwrap();
            let f = File::create(&args[2]).unwrap();
            let mut input = BufReader::new(i);
            let mut file = BufWriter::new(&f);
            let comps = compute(&mut input);
            serialize(comps, &mut file, b'\n');
        },
        2 => {
            let i = File::open(&args[1]).unwrap();
            let mut input = BufReader::new(i);
            let comps = compute(&mut input);
            serialize(comps, &mut BufWriter::new(stdout()), b'\n');
        },
        1 => {
            let comps = compute(&mut BufReader::new(stdin()));
            serialize(comps, &mut BufWriter::new(stdout()), b'\n');
        },
        _ => {
            writeln!(&mut stderr(), "Usage: isol [SOURCE] [[TARGET]]").unwrap();
            process::exit(1);
        }
    }

}
