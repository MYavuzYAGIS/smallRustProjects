use std::{env, fs::File, io::BufRead, io::BufReader, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("readfromfile <wordlist.txt> ");
    };

    let file = File::open(&args[2]).expect("Unable to open");
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line = line.expect("not found line");
        println!("{}", line);
    }
}
