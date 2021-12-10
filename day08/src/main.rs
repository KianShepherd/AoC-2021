use std::fs;

fn count_unique(line: &str) -> usize {
    let nums = line.split(' ').collect::<Vec<&str>>();

    nums.iter()
        .filter(|line| {
            line.trim().len() == 2
                || line.trim().len() == 3
                || line.trim().len() == 4
                || line.trim().len() == 7
        })
        .count()
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.trim().split('\n').collect::<Vec<&str>>();
    //println!("{:?}", v);
    let outputs = v
        .iter()
        .map(|line| line.split('|').collect::<Vec<&str>>()[1].trim())
        .collect::<Vec<&str>>();
    println!(
        "{:?}",
        outputs
            .iter()
            .map(|line| count_unique(line))
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>()
    ); // Part 1
}
