use std::io::Read;
use std::str::FromStr;

fn safe(list: &[i32]) -> bool {
    let list_vec = Vec::from(list);

    let differences: Vec<i32> = list_vec
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect();

    let decreasing = differences.iter().all(|&x| x > 0);
    let increasing = differences.iter().all(|&x| x < 0);

    let increasing_or_decreasing = decreasing || increasing;

    let differ = differences.iter().all(|&x| 1 <= x.abs() && x.abs() <= 3);

    return differ && increasing_or_decreasing;
}

fn new_save(list: &[i32]) -> bool {

    for i in 0..list.len() {
        let (left, right) = list.split_at(i);

        let new_list = [left, &right[1..]].concat();
        if safe(&new_list) {
            return true;
        }
    }

    false
}

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
            let result = s.lines().map(|line| {
                let numbers: Vec<i32> =
                    line.split(" ").map(|x| i32::from_str(x).unwrap()).collect();
                new_save(numbers.as_slice())
            }).filter(|b| *b).count();

            println!("{}", result);
            Ok(())
        }

        Err(_) => Err(()),
    }
}
