use std::fs::File;
use std::io::Read;
use std::path::Prefix::Verbatim;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();

    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    println!("Data {:?}", data);
}
