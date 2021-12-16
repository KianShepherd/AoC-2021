use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input")
       .expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<i32> = contents.split("\n").filter_map(|w| w.parse().ok()).collect();

    let mut prev = v[0];
    let mut count = 0;
    for index in 1..v.len(){
        if prev < v[index] {
            count += 1;
        }
        prev = v[index];
    }
    println!("{:?} : took {:?}", count, start.elapsed());

    let start = Instant::now();
    let v: Vec<i32> = contents.split("\n").filter_map(|w| w.parse().ok()).collect();

    let mut prev = v[0] + v[1] + v[2];
    let mut count = 0;
    for index in 1..v.len() - 2 {
        if prev < (v[index] + v[index + 1] + v[index + 2]) {
            count += 1;
        }
        prev = v[index] + v[index + 1] + v[index + 2];
    }
    println!("{:?} : took {:?}", count, start.elapsed());
}