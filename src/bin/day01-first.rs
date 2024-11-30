use std::io::Read;

fn main() {
    let mut args = std::env::args();

    let s = match args.len() {
        2 => {
            //read file
            let filename = args.nth(1).unwrap();
            std::fs::read_to_string(filename).unwrap()
        }
        _ => {
            //read stdin
            let mut buffer = String::new();
            let _ = std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    println!("{}", s);
}
