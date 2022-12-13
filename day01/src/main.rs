use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./resources/input.txt").expect("File not found!");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file!");

    let splitted_data = data.lines();

    // let mut old: u32 = 0;
    let mut top_three: [u32; 3] = [0, 0, 0];

    let mut buffer: u32 = 0;

    for line in splitted_data {
        if line == "" {
            for it in &mut top_three.iter_mut() {
                if it < &mut buffer {
                    let tmp: u32 = buffer;
                    buffer = *it;
                    *it = tmp;
                }
            }
            buffer = 0;
        } else {
            let tmp: u32 = line.parse().unwrap();
            buffer += tmp;
        }
    }

    // Output greatest calories bag
    println!(
        "The elv with the food containing the most calories has in total {:?} calories!",
        top_three
    );

    let result: u32 = top_three[0] + top_three[1] + top_three[2];
    println!("In total they carry {} calories!", result)
}
