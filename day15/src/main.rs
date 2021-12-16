use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents.trim().split('\n').collect::<Vec<&str>>();

    println!("{:?}", v);
    println!("took {:?}", start.elapsed());
}
