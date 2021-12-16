use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Fish {
    count: i64,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let fishes: Vec<i32> = contents
        .trim()
        .split(',')
        .filter_map(|w| w.parse::<i32>().ok())
        .collect();
    let days = 256;
    let mut fish_struct = vec![];
    for _i in 0..9 {
        fish_struct.push(Fish { count: 0 });
    }
    for fish in fishes {
        fish_struct[fish as usize].count += 1;
    }
    //println!("{:?}", fish_struct);
    for _i in 0..days {
        let new_born_count = fish_struct[0].count;
        for j in 0..8 {
            fish_struct[j].count = fish_struct[j + 1].count;
        }
        fish_struct[6].count += new_born_count;
        fish_struct[8].count = new_born_count;
    }

    println!(
        "{:?} : took {:?}",
        fish_struct.iter().fold(0, |mut sum, fish| {
            sum += fish.count;
            sum
        }),
        start.elapsed()
    );
}
