use std::collections::HashMap;
use std::fs;

fn main() {
    const ROUNDS: usize = 40;
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut v = contents.trim().split('\n');
    let polymer_string = v.next().unwrap().to_string();
    let first_element = polymer_string.chars().next().unwrap();
    let last_element = polymer_string.chars().last().unwrap();
    v.next();
    let insertions = v.collect::<Vec<&str>>();
    //println!("{:?}", polymer_string);
    //println!("{:?}\n\n", insertions);

    let mut polymers = HashMap::<String, u64>::new();
    for i in 0..polymer_string.len() - 1 {
        let mut poly_string = polymer_string.chars().nth(i).unwrap().to_string();
        poly_string.push(polymer_string.chars().nth(i + 1).unwrap());
        let polymer = polymers.entry(poly_string).or_insert(0);
        *polymer += 1;
    }

    let mut insertion_map = HashMap::new();
    for pair in &insertions {
        let mut temp = pair.split(" -> ");
        let poly_pair = temp.next().unwrap();
        let inserted = temp.next().unwrap();
        let mut to_insert = vec![];
        let mut p1 = poly_pair.chars().next().unwrap().to_string();
        p1.push(inserted.chars().next().unwrap());
        to_insert.push(p1);

        let mut p2 = inserted.chars().next().unwrap().to_string();
        p2.push(poly_pair.chars().nth(1).unwrap());
        to_insert.push(p2);
        let insertion_pair = (to_insert[0].clone(), to_insert[1].clone());

        insertion_map.insert(poly_pair, insertion_pair);
    }

    //println!("{:?}\n\n{:?}", polymers, insertion_map);
    for _step in 0..ROUNDS {
        let mut next_polymer = HashMap::new();
        for pair in polymers.keys() {
            let new_polymers = insertion_map.get(&pair as &str).unwrap();
            let polymer_count = polymers.get(pair).unwrap();

            let polymer = next_polymer.entry(new_polymers.0.to_string()).or_insert(0);
            *polymer += polymer_count;

            let polymer = next_polymer.entry(new_polymers.1.to_string()).or_insert(0);
            *polymer += polymer_count;
        }

        polymers = next_polymer;
    }
    //println!("{:?}\n\n{:?}", polymers, insertion_map);

    let mut chars_map = HashMap::new();
    for pair in polymers.keys() {
        let element_one = pair.chars().next().unwrap();
        let element_two = pair.chars().nth(1).unwrap();
        let element_count = polymers.get(pair).unwrap();

        let element = chars_map.entry(element_one).or_insert(0);
        *element += element_count;

        let element = chars_map.entry(element_two).or_insert(0);
        *element += element_count;
    }

    let element = chars_map.entry(first_element).or_insert(0);
    *element += 1;
    let element = chars_map.entry(last_element).or_insert(0);
    *element += 1;

    let mut highest = 0;
    let mut lowest = std::u64::MAX;
    let char_keys = chars_map.clone();
    for key in char_keys.keys() {
        let element = chars_map.entry(*key).or_insert(0);
        *element /= 2;
        if *element < lowest {
            lowest = *element;
        }
        if *element > highest {
            highest = *element;
        }
    }

    //println!("{:?}", chars_map);
    println!("{:?}", highest - lowest); // Part 1 & 2
}
