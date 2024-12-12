use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    blocks: u32,
}

#[derive(Debug, Clone, Copy)]
enum DiskItem {
    File(File),
    FreeSpace(u32),
}

fn contains_free_space(disk: &Vec<DiskItem>) -> bool {
    disk.iter().any(|diskitem| {
        match diskitem {
            DiskItem::FreeSpace(_) => true,
            _ => false,
        }
    })
}

fn checksum(disk: &Vec<DiskItem>) -> u64 {
    let mut total = 0u64;
    let mut k = 0u64;

    for f in disk.iter() {
        match f {
            DiskItem::File(file) => {
                for _ in 0..file.blocks {
                    total += k * (file.id as u64);
                    k += 1;
                }
            }
            _ => panic!()
        }
    } 

    total
}

//assumes that disk contains free space
fn move_files(disk: &mut Vec<DiskItem>) {

    while let DiskItem::FreeSpace(_) = disk.last().unwrap() {
        disk.pop(); 
    }

    if !contains_free_space(&disk) {
        return;
    }

    let id_first_free = disk.iter().position(|disk_item| {
        match disk_item {
            DiskItem::FreeSpace(_) => true, 
            _ => false,
        }
    }).unwrap();

    let n_free = match disk[id_first_free] {
        DiskItem::FreeSpace(n) => n,   
        _ => panic!()
    };

    let last_item = match disk.last().unwrap().clone() {
        DiskItem::File(file) => file,
        _ => panic!()  
    };

    let len = disk.len();

    if n_free == last_item.blocks {
        disk[id_first_free] = DiskItem::File(last_item.clone());
        disk.pop();
    } else if n_free > last_item.blocks {
        let new_free = DiskItem::FreeSpace(n_free - last_item.blocks);        
        disk[id_first_free] = DiskItem::File(last_item);
        disk.insert(id_first_free + 1, new_free);
        disk.pop();
    }
    else { // n_free < last_item.blocks
        let file = DiskItem::File(File {
            id: last_item.id,
            blocks: n_free
        });
        disk[id_first_free] = file;

        let last_file = DiskItem::File(File {
            id: last_item.id,
            blocks: last_item.blocks - n_free,
        });
        disk[len - 1] = last_file;
    }

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
            let mut disk: Vec<DiskItem> = Vec::new();

            let mut next_is_disk = true;
            let mut file_id = 0;

            for c in s.trim().chars() {
                let n = c.to_digit(10).unwrap();
                if next_is_disk {
                    let file = File {
                        id: file_id,
                        blocks: n,
                    };

                    disk.push(DiskItem::File(file));
                    file_id += 1;
                } else {
                    if n > 0 {
                        disk.push(DiskItem::FreeSpace(n));
                    }
                }

                next_is_disk = !next_is_disk;
            }

            while contains_free_space(&disk) {
                move_files(&mut disk); 
            }

            println!("{}", checksum(&disk));
            Ok(())
        }

        Err(_) => Err(()),
    }
}
