use std::{collections::HashSet, fs};
use std::time::Instant;

fn sanatize_points(_points: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut point_set: HashSet<(usize, usize)> = HashSet::new();

    for point in _points {
        point_set.insert(point);
    }

    point_set.into_iter().collect::<Vec<(usize, usize)>>()
}

fn fold_point(
    point: (usize, usize),
    fold_horizontal: bool,
    fold_location: usize,
) -> (usize, usize) {
    if fold_horizontal {
        // fold along y-axis
        if point.1 > fold_location {
            (point.0, fold_location - (point.1 - fold_location))
        } else {
            point
        }
    } else {
        // fold along y-axis
        if point.0 > fold_location {
            (fold_location - (point.0 - fold_location), point.1)
        } else {
            point
        }
    }
}

fn fold_paper(
    _points: Vec<(usize, usize)>,
    fold_horizontal: bool,
    fold_location: usize,
) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for point in _points {
        points.push(fold_point(point, fold_horizontal, fold_location));
    }

    sanatize_points(points)
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents.trim().split('\n').collect::<Vec<&str>>();
    let mut points = vec![];
    let mut folds = vec![];
    let mut on_points = true;

    for _line in v {
        let line = _line.trim();
        if line.is_empty() {
            on_points = false;
            continue;
        }

        if on_points {
            let mut _points = line.split(',');
            let p1 = _points.next().unwrap().parse::<usize>().unwrap();
            let p2 = _points.next().unwrap().parse::<usize>().unwrap();
            points.push((p1, p2));
        } else {
            let fold_info = line.split(' ').nth(2).unwrap();
            let mut fold_information = fold_info.split('=');
            let fold_location = (
                fold_information.next().unwrap() != "x",
                fold_information.next().unwrap().parse::<usize>().unwrap(),
            );
            folds.push(fold_location);
        }
    }

    //println!("{:?}", points.len());
    //println!("{:?}", points);
    //println!("{:?}", folds);
    let first_fold = folds[0];
    println!(
        "{:?} : took {:?}",
        fold_paper(points.clone(), first_fold.0, first_fold.1).len(),
        start.elapsed()
    ); // Part 1
    let start = Instant::now();
    for fold in folds {
        points = fold_paper(points.clone(), fold.0, fold.1);
    }
    let mut max_x = 0;
    let mut max_y = 0;
    for point in &points {
        if point.0 >= max_x {
            max_x = point.0 + 1;
        }
        if point.1 >= max_y {
            max_y = point.1 + 1;
        }
    }
    let mut display = vec![];
    for _y in 0..max_y {
        let mut line = vec![];
        for _x in 0..max_x {
            line.push(" ");
        }
        display.push(line);
    }
    for point in points.clone() {
        display[point.1][point.0] = "#";
    }
    let elapsed = start.elapsed();
    for line in &display {
        for chr in line {
            print!("{}", chr);
        }
        println!();
    } // Part 2
    println!("took {:?}", elapsed);
}
