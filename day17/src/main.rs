use std::fs;
use std::time::Instant;

fn fire_shot(_x_speed: i32, _y_speed: i32, bounding_box: (i32, i32, i32, i32)) -> i32 {
    let mut x_speed = _x_speed;
    let mut y_speed = _y_speed;
    let mut highest_point = 0;
    let mut pos = (0, 0);

    loop {
        pos = (pos.0 + x_speed, pos.1 + y_speed);
        if pos.1 > highest_point {
            highest_point = pos.1;
        }
        if x_speed > 0 {
            x_speed -= 1;
        }
        y_speed -= 1;
        if pos.0 > bounding_box.2 || pos.1 < bounding_box.3 {
            return -35000;
        } else if pos.0 >= bounding_box.0
            && pos.0 <= bounding_box.2
            && pos.1 <= bounding_box.1
            && pos.1 >= bounding_box.3
        {
            break;
        }
    }

    highest_point
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let bounding_box = {
        let mut v = contents.trim().split(' ');
        v.next();
        v.next();
        let mut x_range = &v.next().unwrap()[2..];
        x_range = &x_range[..x_range.len() - 1];
        let min_x = x_range.split("..").next().unwrap().parse::<i32>().unwrap();
        let max_x = x_range.split("..").nth(1).unwrap().parse::<i32>().unwrap();
        let y_range = &v.next().unwrap()[2..];
        let max_y = y_range.split("..").next().unwrap().parse::<i32>().unwrap();
        let min_y = y_range.split("..").nth(1).unwrap().parse::<i32>().unwrap();

        (min_x, min_y, max_x, max_y)
    };
    let mut highest = 0;
    let mut count_hits = 0;
    for x in 0..(bounding_box.2 + 1) {
        for y in (bounding_box.3 - 1)..500 {
            let height = fire_shot(x, y, bounding_box);
            //println!("{:?}", height);
            if height != -35000 {
                count_hits += 1;
            }
            if height > highest {
                highest = height;
            }
        }
    }

    println!(
        "max height -> {:?} : num hits -> {:?} : took {:?}",
        highest,
        count_hits,
        start.elapsed()
    );
}
