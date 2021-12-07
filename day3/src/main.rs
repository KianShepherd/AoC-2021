use std::fs;

#[derive(Debug)]
struct Counter {
    one_count: i32,
    zero_count: i32,
}

fn main() {
    let contents = fs::read_to_string("input")
       .expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split("\n").collect();
    let mut index;
    let mut counters = [
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
        Counter { one_count: 0, zero_count: 0 },
    ];

    for code in v {
        index = 0;
        for chr in code.chars() {
            if chr == '1' {
                counters[index].one_count += 1;
            } else {
                counters[index].zero_count += 1;
            }
            index += 1;
        }
    }

    let mut gamma_string = "".to_string();
    for counter in counters {
        if counter.one_count > counter.zero_count {
            gamma_string += "1";
        } else {
            gamma_string += "0";
        }
    }
    
    gamma_string = gamma_string.chars().rev().collect::<String>();
    let mut gamma_value = 0;
    let mut epsilon_value = 0;
    let chars = gamma_string.chars();
    let mut i = 0;

    for chr in chars {
        if chr == '1' {
            gamma_value += 2_i32.pow(i);
        } else {
            epsilon_value += 2_i32.pow(i);
        }
        i += 1;
    }
    println!("{:?}", gamma_value * epsilon_value);
}
