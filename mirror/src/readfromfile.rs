use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fun main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:");
        println!("file_reader: <wordlist.txt> ");
        return Ok(());
    }

    let file_to_read:File = File::open(&args[1])?;
    let file_reader: BufReader<&File> = BufReader::new(&file_to_read);

    for line:Result<String,Error> in file_reader.lines(){
        println("{}",line);
        return Ok(());
    }
}