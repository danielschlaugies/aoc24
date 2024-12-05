use std::cmp::Ordering;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
struct Page {
    number: i32,
    print_order: Arc<Vec<(i32, i32)>>
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        if self.number == other.number {
            return Some(Ordering::Equal);
        }

        for (a, b) in self.print_order.iter() {
            if *a == self.number && *b == other.number {
                return Some(Ordering::Less);
            }
            else if *a == other.number && *b == self.number {
                return Some(Ordering::Greater);
            }
        }

        None
    }
}

fn in_order(list: &[Page]) ->  bool {
    list.is_sorted_by(|x, y| {
        match x.partial_cmp(y) {
            Some(Ordering::Greater) => false,
            _ => true
        }
    })
}

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

            let mut split = s.split("\n\n");
            let rule_string = split.next().unwrap();
            let mut rules: Vec<(i32, i32)> = Vec::new();

            rule_string.lines().for_each(|line| {
                let mut line_split = line.split("|");

                let a = i32::from_str(line_split.next().unwrap()).unwrap();
                let b = i32::from_str(line_split.next().unwrap()).unwrap();

                rules.push((a, b));
            });

            let page_order = Arc::new(rules);

            let mut total = 0;

            let update_string = split.next().unwrap();
            update_string.lines().for_each(|line| {
                let numbers: Vec<Page> = line.split(',').map(|n| {
                    let num = n.parse::<i32>().unwrap();
                    Page {number: num, print_order: page_order.clone()}
                }).collect();

                if in_order(numbers.as_slice()) {
                    let mid_index = numbers.len() / 2;
                    total += numbers[mid_index].number;
                }

            });

            println!("{}", total);
            Ok(())
        }

        Err(_) => Err(()),
    }
}
