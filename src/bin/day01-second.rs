use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

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

            let mut hm: HashMap<i32, i32> = HashMap::new();
            b.iter().for_each(|x| {
                let n = hm.get(x).unwrap_or(&0);
                hm.insert(*x, n + 1);
            });

            let result: i32 = a.iter().map(|x| {
                let o = hm.get(x).unwrap_or(&0) ;
                x * o
            }).sum();

            println!("{}", result);

            Ok(())
        }

        Err(_) => Err(()),
    }
}
