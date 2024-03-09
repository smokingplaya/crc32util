use crate::print;
use crc32fast::Hasher;
use std::{fs::File, io::Read, path::Path};

fn calc(mut file: File) -> u32 {
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("err");

    let mut hasher = Hasher::new();
    hasher.update(&content);
    hasher.finalize()
}

pub fn check(file_name: String) {
    let path = Path::new(&file_name);

    if !path.exists() {
        return print("File wasn't found!");
    }

    let file = File::open(file_name);

    match file {
        Ok(file) => {
            print("Calculating hash-summ...");

            let result: u32 = calc(file);
            print(&format!("Summ: {}", result));
        }
        Err(_) => {
            print("Can't open file!");
        }
    }
}
