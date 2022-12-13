use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./resources/input.txt").expect("File not found!");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file!");

    let splitted_data = data.lines();

    let mut old: u32 = 0;
    let mut buffer: u32 = 0;

    for line in splitted_data {
        if line == "" {
            if old < buffer {
                old = buffer;
            }
            buffer = 0;
        } else {
            let tmp: u32 = line.parse().unwrap();
            buffer += tmp;
        }
    }

    // Output greatest calories bag
    print!(
        "The elv with the food containing the most calories has in total {} calories!",
        old
    );
}
