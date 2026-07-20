extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;          // BufReader reads a large chunk of file in memory and serves it, reducing the expensive system calls required
use std::time::Instant;         // to show the time required to compress a file

fn main() {
    // the program expects three args, i.e., source, target
    if args().len() != 3 {
        // eprint! prints to stderr instead of stdout of print!
        eprintln!("Usage: `source` `target`");
        return;
    }
    // File::open("input.txt") return a Result<>, therefore unwarp() it
    // agrs().nth(1) returns an Option<>, therefore unwrap() it
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());
}