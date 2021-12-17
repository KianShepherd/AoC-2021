use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn find_all_paths_p1(map: &HashMap<&str, Vec<&str>>) -> u32 {
    find_all_paths_tail_p1(map, "start", &vec![])
}

fn can_visit_p1(node: &str, visited: &Vec<&str>) -> bool {
    if node == "start" {
        return false;
    }
    let mut all_lower = true;
    for chr in node.chars() {
        if chr.is_uppercase() {
            all_lower = false;
        }
    }

    if all_lower {
        !visited.contains(&node)
    } else {
        true
    }
}

fn find_all_paths_tail_p1(
    map: &HashMap<&str, Vec<&str>>,
    current_node: &str,
    visited: &Vec<&str>,
) -> u32 {
    if current_node == "end" {
        return 1;
    }
    let mut visited_new = visited.clone();
    visited_new.push(current_node);
    let mut sum = 0;
    let edges = map
        .get(current_node)
        .unwrap()
        .iter()
        .filter(|node| can_visit_p1(node, visited))
        .map(|node| *node)
        .collect::<Vec<&str>>();
    if edges.is_empty() {
        return 0;
    }
    for edge in edges {
        sum += find_all_paths_tail_p1(map, edge, &visited_new);
    }

    sum
}

fn find_all_paths(map: &HashMap<&str, Vec<&str>>) -> u32 {
    find_all_paths_tail(map, "start", &vec![])
}

fn contains_two_duplicate_small_caves(node: &str, _visited: &Vec<&str>) -> bool {
    let mut visited = _visited.clone();
    visited.push(node);
    let small_caves = visited
        .clone()
        .iter()
        .filter(|cave| {
            for chr in cave.chars() {
                if chr.is_uppercase() {
                    return false;
                }
            }
            true
        })
        .map(|cave| *cave)
        .collect::<Vec<&str>>();
    let mut duplicate_found = false;
    let mut caves = HashMap::new();
    //println!("caves : {:?}", small_caves);
    for cave in &small_caves {
        let entry = caves.entry(cave).or_insert(0);
        *entry += 1;
    }

    let caves = caves
        .iter()
        .filter(|(k, v)| **v > 1)
        .map(|(k, v)| *v)
        .collect::<Vec<i32>>();
    if caves.len() > 1 {
        return false;
    }
    for count in caves {
        if count > 2 {
            return false;
        }
    }

    true
}

fn can_visit(node: &str, visited: &Vec<&str>) -> bool {
    if node == "start" {
        return false;
    }
    let mut all_lower = true;
    for chr in node.chars() {
        if chr.is_uppercase() {
            all_lower = false;
        }
    }

    if all_lower {
        if contains_two_duplicate_small_caves(node, visited) {
            true
        } else {
            false
        }
    } else {
        true
    }
}

fn find_all_paths_tail(
    map: &HashMap<&str, Vec<&str>>,
    current_node: &str,
    visited: &Vec<&str>,
) -> u32 {
    let mut visited_new = visited.clone();
    visited_new.push(current_node);
    let mut sum = 0;
    if current_node == "end" {
        //println!("{:?}", visited_new);
        return 1;
    }
    let edges = map
        .get(current_node)
        .unwrap()
        .iter()
        .filter(|node| can_visit(node, &visited_new))
        .map(|node| *node)
        .collect::<Vec<&str>>();
    //println!("{:?}", edges);
    if edges.is_empty() {
        return 0;
    }
    for edge in edges {
        sum += find_all_paths_tail(map, edge, &visited_new);
    }

    sum
}
fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut graph = HashMap::new();
    for line in &v {
        let node = line.split('-').next().unwrap();
        let value = line.split('-').nth(1).unwrap();
        let entry = graph.entry(node).or_insert_with(Vec::new);
        (*entry).push(value);
        let entry = graph.entry(value).or_insert_with(Vec::new);
        (*entry).push(node);
    }

    //println!("{:?}", graph);
    println!(
        "{:?} : took {:?}",
        find_all_paths_p1(&graph),
        start.elapsed()
    );

    let start = Instant::now();
    let v: Vec<&str> = contents
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut graph = HashMap::new();
    for line in &v {
        let node = line.split('-').next().unwrap();
        let value = line.split('-').nth(1).unwrap();
        let entry = graph.entry(node).or_insert_with(Vec::new);
        (*entry).push(value);
        let entry = graph.entry(value).or_insert_with(Vec::new);
        (*entry).push(node);
    }

    //println!("{:?}", graph);
    println!("{:?} : took {:?}", find_all_paths(&graph), start.elapsed());
}
