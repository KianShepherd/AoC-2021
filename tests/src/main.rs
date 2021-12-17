use std::collections::HashMap;

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

fn main() {
    let mut vals = vec!["dc", "kj", "kj", "B"];
    println!("{:?}", contains_two_duplicate_small_caves("dc", &vals));
}
