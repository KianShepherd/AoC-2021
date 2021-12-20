use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();


    println!("took {:?}", start.elapsed());
}
