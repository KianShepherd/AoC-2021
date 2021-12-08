use std::fs;

#[derive(Debug)]
struct Counter {
    one_count: i32,
    zero_count: i32,
}

fn filter_arr(nums: Vec<Vec<i32>>, index: usize, looking_for: i32) {}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split('\n').collect();
    let mut values = vec![];
    for line in v {
        let mut nums = vec![];
        for chr in line.chars() {
            if chr == '1' {
                nums.push(1);
            } else {
                nums.push(0);
            }
        }
        if nums.len() > 1 {
            values.push(nums);
        }
    }
    println!("{:?}", values);
}
