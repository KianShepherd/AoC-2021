use std::fs;
use std::time::Instant;

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

fn get_four_signal(line: &Vec<&str>) -> String {
    line.clone()
        .into_iter()
        .filter(|line| line.len() == 4)
        .collect::<Vec<&str>>()[0]
        .to_string()
}

fn get_one_signal(line: &Vec<&str>) -> String {
    line.clone()
        .into_iter()
        .filter(|line| line.len() == 2)
        .collect::<Vec<&str>>()[0]
        .to_string()
}

fn overlap(signal: &str, ref_signal: &str) -> usize {
    let mut counter = 0;
    let s1 = signal.to_string();
    let s2 = ref_signal.to_string();
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                counter += 1;
                break;
            }
        }
    }
    counter
}

fn decode_signal(signal: &str, one_sig: &str, four_sig: &str) -> u32 {
    let signal_length = signal.len();
    if signal_length == 2 {
        return 1;
    } else if signal_length == 4 {
        return 4;
    } else if signal_length == 3 {
        return 7;
    } else if signal_length == 7 {
        return 8;
    } else if signal_length == 5 {
        if overlap(signal, one_sig) == 2 {
            return 3;
        } else if overlap(signal, four_sig) == 3 {
            return 5;
        } else {
            return 2;
        }
    } else if signal_length == 6 {
        if overlap(signal, one_sig) == 1 {
            return 6;
        } else if overlap(signal, four_sig) == 4 {
            return 9;
        } else {
            return 0;
        }
    }
    0
}

fn get_output_value(line: Vec<&str>) -> u32 {
    let one_sig = get_one_signal(&line);
    let four_sig = get_four_signal(&line);
    (decode_signal(line[line.len() - 4], &one_sig, &four_sig) * 1000)
        + (decode_signal(line[line.len() - 3], &one_sig, &four_sig) * 100)
        + (decode_signal(line[line.len() - 2], &one_sig, &four_sig) * 10)
        + decode_signal(line[line.len() - 1], &one_sig, &four_sig)
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents.trim().split('\n').collect::<Vec<&str>>();
    //println!("{:?}", v);
    let outputs = v
        .iter()
        .map(|line| line.split('|').collect::<Vec<&str>>()[1].trim())
        .collect::<Vec<&str>>();
    println!(
        "{:?} : took {:?}",
        outputs
            .iter()
            .map(|line| count_unique(line))
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>(),
            start.elapsed()
    ); // Part 1
    let start = Instant::now();
    let lines_ = v
        .iter()
        .map(|line| line.split('|').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut lines = vec![];
    for line in lines_ {
        let mut sub_line_signals = vec![];
        for sub_line in line {
            let mut sub = sub_line.trim().split(' ').collect::<Vec<&str>>();
            sub_line_signals.append(&mut sub);
        }
        lines.push(sub_line_signals);
    }

    println!(
        "{:?} : took {:?}",
        lines
            .iter()
            .map(|line| get_output_value(line.to_vec()))
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>(),
            start.elapsed()
    );
}
