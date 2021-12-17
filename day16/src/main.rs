use std::fs;
use std::time::Instant;

fn get_binary_from_hex(hex_val: char) -> String {
    match hex_val {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => "".to_string(),
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v = contents.trim().chars();
    let mut binary = "".to_string();
    for chr in v {
        println!("{:?}", chr);
        binary.push_str(&get_binary_from_hex(chr));
    }

    println!("{:?}", binary);
    println!("took {:?}", start.elapsed());
}
