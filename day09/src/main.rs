use std::fs;
use std::time::Instant;

fn get_neighbours(heights: &[Vec<u32>], x_pos: usize, y_pos: usize) -> Vec<Vec<usize>> {
    let mut neighbours = vec![];
    if y_pos >= 1 && heights[y_pos - 1][x_pos] != 9 {
        neighbours.push(vec![x_pos, y_pos - 1]);
    }
    if y_pos <= heights.len() - 2 && heights[y_pos + 1][x_pos] != 9 {
        neighbours.push(vec![x_pos, y_pos + 1]);
    }
    if x_pos >= 1 && heights[y_pos][x_pos - 1] != 9 {
        neighbours.push(vec![x_pos - 1, y_pos]);
    }
    if x_pos <= heights.len() - 2 && heights[y_pos][x_pos + 1] != 9 {
        neighbours.push(vec![x_pos + 1, y_pos]);
    }

    neighbours
}

fn get_basin_size(heights: &[Vec<u32>], x_pos: usize, y_pos: usize) -> usize {
    let mut basin_indexes = vec![vec![x_pos, y_pos]];
    let mut new_additions = vec![vec![x_pos, y_pos]];
    loop {
        let mut temp = vec![];
        for point in &new_additions {
            let neighbours = get_neighbours(heights, point[0], point[1]);
            for neighbour in &neighbours {
                let mut in_basin = false;
                for point in &basin_indexes {
                    if point[0] == neighbour[0] && point[1] == neighbour[1] {
                        in_basin = true;
                    }
                }
                if !in_basin {
                    basin_indexes.push(vec![neighbour[0], neighbour[1]]);
                    temp.push(vec![neighbour[0], neighbour[1]]);
                }
            }
        }
        new_additions = temp;
        if new_additions.is_empty() {
            break;
        }
    }

    basin_indexes.len()
}

fn check_neighbours(heights: &[Vec<u32>], x_pos: usize, y_pos: usize, height: u32) -> bool {
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
    let start = Instant::now();
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
        "{:?} : took {:?}",
        low_point_locations
            .iter()
            .map(|(x, y)| *v.get(*y).unwrap().get(*x).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .fold(0, |sum, num| { sum + num + 1 }),
            start.elapsed()
    ); // Part 1
       //println!("{:?}", low_point_locations);
    let start = Instant::now();
    let mut basin_sizes = low_point_locations
        .iter()
        .map(|(x, y)| get_basin_size(&v, *x, *y))
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    println!("{:?} : took {:?}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2], start.elapsed());
}
