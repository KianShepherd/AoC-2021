use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn num_arr_to_edges(nums: &Vec<Vec<usize>>) -> Vec<Vec<Edge>> {
    let mut edge_list = vec![];
    for y in 0..nums.len() {
        for x in 0..nums[y].len() {
            let mut edges = vec![];
            if y > 0 {
                edges.push(Edge {
                    node: x + ((y - 1) * nums[y - 1].len()),
                    cost: nums[y - 1][x],
                });
            }
            if y < nums.len() - 1 {
                edges.push(Edge {
                    node: x + ((y + 1) * nums[y + 1].len()),
                    cost: nums[y + 1][x],
                });
            }
            if x > 0 {
                edges.push(Edge {
                    node: (x - 1) + (y * nums[y].len()),
                    cost: nums[y][x - 1],
                });
            }
            if x < nums.len() - 1 {
                edges.push(Edge {
                    node: (x + 1) + (y * nums[y].len()),
                    cost: nums[y][x + 1],
                });
            }

            edge_list.push(edges);
        }
    }

    edge_list
}

fn extend_cave(nums: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut numbers = vec![];
    for y_scale in 0..5 {
        for y in 0..nums.len() {
            let mut line = vec![];
            for x_scale in 0..5 {
                for x in 0..nums[y].len() {
                    let mut new_score = nums[y][x] + x_scale + y_scale;
                    if new_score >= 10 {
                        new_score -= 9;
                    }
                    line.push(new_score);
                }
            }
            numbers.push(line);
        }
    }
    numbers
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<Vec<usize>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let edges = num_arr_to_edges(&v);
    let final_edge = (v.len() * v[0].len()) - 1;
    //println!("{:?}", edges);

    println!(
        "{:?} : took {:?}",
        shortest_path(&edges, 0, final_edge),
        start.elapsed()
    );

    let start = Instant::now();
    let v: Vec<Vec<usize>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let new_danger_levels = extend_cave(&v);
    //println!("{:?}", new_danger_levels);
    let edges = num_arr_to_edges(&new_danger_levels);
    let final_edge = (new_danger_levels.len() * new_danger_levels[0].len()) - 1;
    //println!("{:?}", edges);

    println!(
        "{:?} : took {:?}",
        shortest_path(&edges, 0, final_edge),
        start.elapsed()
    );
    //println!("{:?}", v);
}
