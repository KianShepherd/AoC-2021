use std::fs;
use std::time::Instant;

fn check_for_errors(line: &str) -> usize {
    let mut brackets = vec![];
    for chr in line.chars() {
        //println!("{:?}", brackets);
        //println!("{:?}", chr);
        if chr == '[' || chr == '<' || chr == '(' || chr == '{' {
            brackets.push(chr);
        } else {
            if chr == ']' && brackets[brackets.len() - 1] == '[' {
                brackets.pop();
            } else if chr == '>' && brackets[brackets.len() - 1] == '<' {
                brackets.pop();
            } else if chr == ')' && brackets[brackets.len() - 1] == '(' {
                brackets.pop();
            } else if chr == '}' && brackets[brackets.len() - 1] == '{' {
                brackets.pop();
            } else {
                if chr == ']' {
                    return 57;
                } else if chr == '>' {
                    return 25137;
                } else if chr == ')' {
                    return 3;
                } else if chr == '}' {
                    return 1197;
                }
            }
        }
    }
    0
}

fn get_incomplete_lines(line: &str) -> Vec<char> {
    let mut brackets = vec![];
    for chr in line.chars() {
        //println!("{:?}", brackets);
        //println!("{:?}", chr);
        if chr == '[' || chr == '<' || chr == '(' || chr == '{' {
            brackets.push(chr);
        } else {
            if chr == ']' && brackets[brackets.len() - 1] == '[' {
                brackets.pop();
            } else if chr == '>' && brackets[brackets.len() - 1] == '<' {
                brackets.pop();
            } else if chr == ')' && brackets[brackets.len() - 1] == '(' {
                brackets.pop();
            } else if chr == '}' && brackets[brackets.len() - 1] == '{' {
                brackets.pop();
            }
        }
    }
    brackets.reverse();
    brackets
}

fn calculate_score(brackets: String) -> u64 {
    let mut score = 0;
    for chr in brackets.chars() {
        score *= 5;
        if chr == '[' {
            score += 2;
        } else if chr == '(' {
            score += 1;
        } else if chr == '{' {
            score += 3;
        } else if chr == '<' {
            score += 4;
        }
    }
    score
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents.trim().split('\n').collect::<Vec<&str>>();

    println!(
        "{:?} : took {:?}",
        v.iter().map(|line| check_for_errors(line)).sum::<usize>(),
        start.elapsed()
    ); //part 1

    let start = Instant::now();
    let incomplete_line = v
        .iter()
        .filter(|line| check_for_errors(*line) == 0)
        .map(|line| *line);
    let mut scores = incomplete_line
        .into_iter()
        .map(|line| get_incomplete_lines(line).into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|line| calculate_score(line.to_string()))
        .collect::<Vec<u64>>();
    scores.sort_unstable();
    println!("{:?} : took {:?}", scores[((scores.len() as f64 / 2.0) - 0.5) as usize], start.elapsed());
}

