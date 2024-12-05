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
            let rows: i32 = lines.len() as i32;
            let cols = lines.first().unwrap().len() as i32;

            let xmas = "XMAS";

            let directions = vec![
                (1, 0),   //right
                (-1, 0),  //left
                (0, 1),   //down
                (0, -1),  //up
                (1, 1),   //down-right
                (1, -1),  //up-right
                (-1, 1),  //down-left
                (-1, -1), //up-left
            ];

            let mut total = 0;

            for row in 0..rows {
                for col in 0..cols {
                    for (i, j) in directions.iter() {
                        let n = row;
                        let m = col;

                        let mut b = true;
                        for l in 0..(xmas.len() as i32) {
                            let nn = n + l * i;
                            let mm = m + l * j;
                            if nn < rows && mm < cols && nn >= 0 && mm >= 0 {
                                if lines[nn as usize].chars().nth(mm as usize).unwrap()
                                    != xmas.chars().nth(l as usize).unwrap()
                                {
                                    b = false;
                                }
                            } else {
                                b = false;
                            }
                        }

                        if b {
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
