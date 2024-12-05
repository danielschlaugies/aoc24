use std::io::Read;

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
            let lines: Vec<&str> = s.lines().collect();
            let mut total = 0;
            let rows = lines.len();
            let cols = lines[0].len();

            for row in 1..(rows - 1) {
                for col in 1..(cols - 1) {
                    if lines[row].chars().nth(col).unwrap() == 'A' {
                        // M M
                        //  A
                        // S S
                        if lines[row - 1].chars().nth(col - 1).unwrap() == 'M'
                            && lines[row - 1].chars().nth(col + 1).unwrap() == 'M'
                            && lines[row + 1].chars().nth(col - 1).unwrap() == 'S'
                            && lines[row + 1].chars().nth(col + 1).unwrap() == 'S'
                        {
                            total += 1;
                        }

                        // S M
                        //  A
                        // S M
                        if lines[row - 1].chars().nth(col - 1).unwrap() == 'S'
                            && lines[row - 1].chars().nth(col + 1).unwrap() == 'M'
                            && lines[row + 1].chars().nth(col - 1).unwrap() == 'S'
                            && lines[row + 1].chars().nth(col + 1).unwrap() == 'M'
                        {
                            total += 1;
                        }

                        // S S
                        //  A
                        // M M
                        if lines[row - 1].chars().nth(col - 1).unwrap() == 'S'
                            && lines[row - 1].chars().nth(col + 1).unwrap() == 'S'
                            && lines[row + 1].chars().nth(col - 1).unwrap() == 'M'
                            && lines[row + 1].chars().nth(col + 1).unwrap() == 'M'
                        {
                            total += 1;
                        }

                        // M S
                        //  A
                        // M S
                        if lines[row - 1].chars().nth(col - 1).unwrap() == 'M'
                            && lines[row - 1].chars().nth(col + 1).unwrap() == 'S'
                            && lines[row + 1].chars().nth(col - 1).unwrap() == 'M'
                            && lines[row + 1].chars().nth(col + 1).unwrap() == 'S'
                        {
                            total += 1;
                        }
                    }
                }
            }

            println!("{}", total);
            Ok(())
        }

        Err(_) => Err(()),
    }
}
