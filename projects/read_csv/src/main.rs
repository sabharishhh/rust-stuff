use csv;
use std::error::Error;
use std::env;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = match csv::Reader::from_path(path) {
        Ok(r) => r,
        Err(e) => return Err(e.into()),
    };

    for result in reader.records() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => return Err(e.into()),
        };

        println!("{:?}", record)
    }

    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: `<filename>.csv`");
        return;
    }

    let filename = args.get(1).unwrap();

    if let Err(e) = read_from_file(filename) {
        eprintln!("{}", e);
    }
}