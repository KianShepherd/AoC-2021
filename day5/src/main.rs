use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let v: Vec<&str> = contents
        .split('\n')
        .filter(|value| !value.is_empty())
        .collect();
    println!("{:?}", v);
}
