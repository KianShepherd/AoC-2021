use std::fs;

fn calc_cost(move_to: i32, location: i32) -> i32 {
    let total_distance = (location - move_to).abs();
    (total_distance * (total_distance + 1)) / 2
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let crab_locations: Vec<i32> = {
        let mut _crab_locations: Vec<i32> = contents
            .trim()
            .split(',')
            .filter_map(|w| w.parse::<i32>().ok())
            .collect();
        _crab_locations.sort_unstable();
        _crab_locations
    };
    let min_location = crab_locations[0];
    let max_location = crab_locations[crab_locations.len() - 1];
    println!(
        "{:?}\n{:?}\n{:?}",
        crab_locations, min_location, max_location
    );
    let mut fuel_costs = vec![];
    for i in min_location..max_location {
        fuel_costs.push(
            crab_locations
                .iter()
                .map(|location| calc_cost(i, *location))
                .fold(0, |mut sum, cost| {
                    sum += cost;
                    sum
                }),
        );
    }
    //println!("{:?}", fuel_costs);
    let mut min = i32::MAX;
    for fuel_cost in fuel_costs {
        if fuel_cost < min {
            min = fuel_cost;
        }
    }
    println!("{:?}", min);
}
