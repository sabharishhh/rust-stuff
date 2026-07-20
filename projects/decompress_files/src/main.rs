use std::fs;
use std::io;
use std::path;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect(); // .collect() collects the data into a collection <_> and the underscore is for the compiler to figure out the type of data being stored in the collection, which is String in this case
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = path::Path::new(&*args[1]); // borrowed view of path string stored as filesystem path
    let file = fs::File::open(&fname).unwrap(); // create a file handle at the indicated path

    let mut archive = zip::ZipArchive::new(file).unwrap(); // create a zip archive reader and creates zip archive object to access contents

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        
    }
    0
}