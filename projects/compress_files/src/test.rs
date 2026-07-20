extern crate flate2;

use std::io::copy;
use std::io::BufReader;
use std::fs::File;
use std::env::agrs;
use std::Instant;
use flate2::GzEncoder;
use flate2::Compression;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwarp()).unwarp());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwarp();
    let output = encoder.finish().unwarp();
}