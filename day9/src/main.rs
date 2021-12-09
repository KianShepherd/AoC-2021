use std::fs;

fn check_neighbours(heights: &Vec<Vec<u32>>, x_pos: usize, y_pos: usize, height: u32) -> bool {
    let neighbour_positions = {
        let mut neighbours = vec![];
        if y_pos >= 1 {
            neighbours.push(vec![x_pos, y_pos - 1]);
        }
        if y_pos <= heights.len() - 2 {
            neighbours.push(vec![x_pos, y_pos + 1]);
        }
        if x_pos >= 1 {
            neighbours.push(vec![x_pos - 1, y_pos]);
        }
        if x_pos <= heights.len() - 2 {
            neighbours.push(vec![x_pos + 1, y_pos]);
        }
        neighbours
    };
    for neighbour in neighbour_positions {
        if heights[neighbour[1]][neighbour[0]] <= height {
            return false;
        }
    }
    true
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    const RADIX: u32 = 10;
    let v: Vec<Vec<u32>> = contents
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|nums| {
            nums.chars()
                .into_iter()
                .map(|num| num.to_digit(RADIX).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut low_point_locations = vec![];
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if check_neighbours(&v, j, i, v[i][j]) {
                low_point_locations.push((j, i));
            }
        }
    }
    println!(
        "{:?}",
        low_point_locations
            .iter()
            .map(|(x, y)| *v.get(*y).unwrap().get(*x).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .fold(0, |sum, num| { sum + num + 1 })
    ); // Part 1
    println!("{:?}", low_point_locations);
}
