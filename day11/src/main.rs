use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct DumboOctopus {
    counter: usize,
    has_flashed: bool,
}

fn get_neighbour_coords(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    for yn in y..y + 3 {
        for xn in x..x + 3 {
            if xn == 0 || yn == 0 || xn > 10 || yn > 10 || (x == xn - 1 && y == yn - 1) {
                continue;
            }
            neighbours.push((xn - 1, yn - 1))
        }
    }
    neighbours
}

impl DumboOctopus {
    fn reset_score(&self) -> DumboOctopus {
        DumboOctopus {
            counter: {
                if self.counter > 9 {
                    0
                } else {
                    self.counter
                }
            },
            has_flashed: false,
        }
    }
}

fn main() {
    const RADIX: u32 = 10;
    const STEPS: usize = 100;

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let mut v: Vec<Vec<DumboOctopus>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|chr| DumboOctopus {
                    counter: chr.to_digit(RADIX).unwrap() as usize,
                    has_flashed: false,
                })
                .collect::<Vec<DumboOctopus>>()
        })
        .collect::<Vec<Vec<DumboOctopus>>>();

    //println!("{:?}", v);
    let mut total_flashes = 0;

    for _i in 0..STEPS {
        let mut first_loop = true;
        loop {
            let mut flashed_this_loop = 0;
            for x in 0..v[0].len() {
                for y in 0..v.len() {
                    if first_loop {
                        v[y][x].counter += 1;
                    }
                    if !v[y][x].has_flashed && v[y][x].counter > 9 {
                        for (xn, yn) in get_neighbour_coords(x, y) {
                            v[yn][xn].counter += 1;
                        }
                        v[y][x].has_flashed = true;
                        total_flashes += 1;
                        flashed_this_loop += 1;
                    }
                }
            }
            first_loop = false;
            if flashed_this_loop == 0 {
                break;
            }
        }

        for x in 0..v[0].len() {
            for y in 0..v.len() {
                v[y][x] = v[y][x].reset_score();
            }
        }
    }

    println!("{:?} : took {:?}", total_flashes, start.elapsed()); // Part 1

    let start = Instant::now();
    let mut v: Vec<Vec<DumboOctopus>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|chr| DumboOctopus {
                    counter: chr.to_digit(RADIX).unwrap() as usize,
                    has_flashed: false,
                })
                .collect::<Vec<DumboOctopus>>()
        })
        .collect::<Vec<Vec<DumboOctopus>>>();

    let mut loop_counter = 0;
    loop {
        let mut first_loop = true;
        let mut flashed_this_round = 0;
        loop {
            let mut flashed_this_loop = 0;
            for x in 0..v[0].len() {
                for y in 0..v.len() {
                    if first_loop {
                        v[y][x].counter += 1;
                    }
                    if !v[y][x].has_flashed && v[y][x].counter > 9 {
                        for (xn, yn) in get_neighbour_coords(x, y) {
                            v[yn][xn].counter += 1;
                        }
                        v[y][x].has_flashed = true;
                        flashed_this_round += 1;
                        flashed_this_loop += 1;
                    }
                }
            }
            first_loop = false;
            if flashed_this_loop == 0 {
                break;
            }
        }
        for x in 0..v[0].len() {
            for y in 0..v.len() {
                v[y][x] = v[y][x].reset_score();
            }
        }
        loop_counter += 1;

        if flashed_this_round == 100 {
            break;
        }
    }

    println!("{:?} : took {:?}", loop_counter, start.elapsed()); // Part 2
}
