use std::collections::HashSet;
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
            let rows = lines.len() as i32;
            let cols = lines[0].len() as i32;

            let mut pos = (0, 0);

            for row in 0..rows {
                for col in 0..cols {
                    if lines[row as usize].chars().nth(col as usize).unwrap() == '^' {
                        pos = (row, col);
                        break;
                    }
                }
            }


            let mut visited: HashSet<(i32, i32)> = HashSet::new();

            let mut directions  = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
            let mut direction = directions.next().unwrap();

            visited.insert(pos);

            while 0 <= pos.0 && pos.0 < rows && 0 <= pos.1 && pos.1 < cols {
                let new_row = pos.0 + direction.0;
                let new_col = pos.1 + direction.1;
                if 0 <= new_row && new_row < rows && 0 <= new_col && new_col < cols {
                    if lines[new_row as usize].chars().nth(new_col as usize).unwrap() == '#' {
                        direction = directions.next().unwrap();
                    }
                    else {
                        pos = (new_row, new_col);
                        visited.insert(pos);
                    }
                }
                else {
                    pos = (new_row, new_col);
                }
            }

            println!("{:?}", visited.len());

            Ok(())
        }

        Err(_) => Err(()),
    }
}
