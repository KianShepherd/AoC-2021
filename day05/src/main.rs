use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(coord_string: &str) -> Self {
        let coord_split: Vec<&str> = coord_string.split(',').collect();
        Self {
            x: coord_split[0].trim().parse::<i32>().unwrap(),
            y: coord_split[1].trim().parse::<i32>().unwrap(),
        }
    }

    fn to_string(&self) -> String {
        self.x.to_string() + "," + &self.y.to_string()
    }

    fn make_range(&self, coord: Coordinate) -> Vec<Coordinate> {
        let mut range = vec![];
        if self.x == coord.x {
            let y1 = cmp::min(self.y, coord.y);
            let y2 = cmp::max(self.y, coord.y) + 1;
            for index in y1..y2 {
                range.push(Coordinate {
                    x: self.x,
                    y: index,
                });
            }
        } else if self.y == coord.y {
            let x1 = cmp::min(self.x, coord.x);
            let x2 = cmp::max(self.x, coord.x) + 1;
            for index in x1..x2 {
                range.push(Coordinate {
                    x: index,
                    y: self.y,
                });
            }
        } else {
            let step_dir_horizontal = if self.x > coord.x { -1 } else { 1 };
            let step_dir_vertical = if self.y > coord.y { -1 } else { 1 };
            let magnitude = (self.x - coord.x).abs() + 1;
            for i in 0..magnitude {
                range.push(Coordinate {
                    x: self.x + (i * step_dir_horizontal),
                    y: self.y + (i * step_dir_vertical),
                });
            }
        }

        range
    }

    fn make_range_p1(&self, coord: Coordinate) -> Vec<Coordinate> {
        let mut range = vec![];
        if self.x == coord.x {
            let y1 = cmp::min(self.y, coord.y);
            let y2 = cmp::max(self.y, coord.y) + 1;
            for index in y1..y2 {
                range.push(Coordinate {
                    x: self.x,
                    y: index,
                });
            }
        } else if self.y == coord.y {
            let x1 = cmp::min(self.x, coord.x);
            let x2 = cmp::max(self.x, coord.x) + 1;
            for index in x1..x2 {
                range.push(Coordinate {
                    x: index,
                    y: self.y,
                });
            }
        }

        range
    }
}

fn main() {
    let contents = fs::read_to_string("input1").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents
        .split('\n')
        .filter(|value| !value.is_empty())
        .collect();
    //println!("{:?}", v);

    let mut path_map = HashMap::new();
    for line in v {
        let line_split: Vec<&str> = line.split(" -> ").collect();
        let first_coord = Coordinate::new(line_split[0]);
        let second_coord = Coordinate::new(line_split[1]);
        let range = first_coord.make_range(second_coord);
        //println!("{:?}", range);
        for coord in range {
            if let std::collections::hash_map::Entry::Vacant(e) = path_map.entry(coord.to_string())
            {
                e.insert(1);
            } else {
                *path_map.get_mut(&coord.to_string()).unwrap() += 1;
            }
        }
    }

    //println!("{:?}", path_map);
    //println!("{:?}", path_map.len());
    path_map = path_map
        .into_iter()
        .filter(|(_k, v)| *v > 1)
        .collect::<HashMap<String, i32>>();
    println!("{:?} : took {:?}", path_map.len(), start.elapsed());
    //println!("{:?}", path_map);
}
