mod crc32;

use std::io::{self, Write};

fn print(message: &str) {
    println!("[crc32util] {}", message);
}

fn main() {
    print("hi there, enter name of the file that you wanna get CRC32 hash summ");

    let mut file_name: String = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut file_name).expect("err");

    crc32::check(file_name.trim().replace("\"", ""));
}
