use std::io::Read;
use std::str::FromStr;

//noinspection DuplicatedCode
fn main() -> Result<(), ()> {
    let mut args = std::env::args();

    let s = match args.len() {
        2 => {
            //read file
            let filename = args.nth(1).unwrap();
            std::fs::read_to_string(filename)
        }
        _ => {
            //read stdin
            let mut buffer = String::new();
            match std::io::stdin().read_to_string(&mut buffer) {
                Ok(_) => Ok(buffer),

                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "could not read stdin",
                )),
            }
        }
    };

    match s {
        Ok(s) => {
            let mut a = Vec::new();
            let mut b = Vec::new();

            s.split("\n").for_each(|line| {

                if line.len() > 0 {
                    let mut l = line.split("   ");

                    let first = i32::from_str(l.next().unwrap()).unwrap();
                    let second = i32::from_str(l.next().unwrap()).unwrap();

                    a.push(first);
                    b.push(second);
                }
            });

            a.sort();
            b.sort();

            let iter = a.iter().zip(b.iter());
            let result: i32 = iter.map(|(x, y)| (x - y).abs()).sum();
            println!("{}", result);
            Ok(())
        }

        Err(_) => Err(()),
    }
}
