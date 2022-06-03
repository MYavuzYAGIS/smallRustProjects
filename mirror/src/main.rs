use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Supply minimum 2 arguments");
    }

    println!("reading your args back to you");
    for line in args {
        println!("{}", line);
    }
}
