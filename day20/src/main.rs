use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
struct MapKey {
    x: i32,
    y: i32,
    key: String,
}

impl MapKey {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            key: x.to_string() + "," + &y.to_string(),
        }
    }

    fn get_key(&self) -> String {
        self.key.to_string()
    }

    fn get_neighbours(&self) -> Vec<MapKey> {
        let mut output = vec![];
        for y in (self.y as i32 - 1)..(self.y as i32 + 2) {
            for x in (self.x as i32 - 1)..(self.x as i32 + 2) {
                output.push(MapKey::new(x, y));
            }
        }

        output
    }
}

#[derive(Debug)]
struct Image {
    pixels: HashMap<String, usize>,
    enhancement_algorithm: String,
    width: usize,
    height: usize,
}

impl Image {
    fn new(_input_image: Vec<&str>, enhancement_algorithm: &str) -> Self {
        let mut input_image = vec![];
        let mut top_bottom_padding = vec![];
        for _ in 0.._input_image[0].len() + 1 {
            top_bottom_padding.push('.');
        }
        input_image.push(top_bottom_padding.clone());

        for _line in _input_image {
            let line = _line.trim();
            let mut new_line = vec![];
            new_line.push('.');

            for chr in line.chars() {
                new_line.push(chr);
            }

            new_line.push('.');
            input_image.push(new_line);
        }

        input_image.push(top_bottom_padding.clone());

        let mut map = HashMap::new();
        let mut keys = vec![];

        for (y, line) in input_image.iter().enumerate() {
            for (x, chr) in line.iter().enumerate() {
                let mapkey = MapKey::new(x as i32, y as i32);
                map.insert(mapkey.get_key(), if *chr == '#' { 1 } else { 0 });
                keys.push(mapkey);
            }
        }

        Self {
            pixels: map,
            enhancement_algorithm: enhancement_algorithm.to_string(),
            width: input_image[0].len(),
            height: input_image.len(),
        }
    }

    fn get_at(&mut self, entry: &MapKey) -> usize {
        if self.pixels.contains_key(&entry.get_key()) {
            let value = self.pixels.entry(entry.get_key()).or_insert(0);
            *value
        } else {
            self.pixels.insert(entry.get_key(), 0);
            0
        }
    }

    fn enhance(&mut self) {
        let mut new_map = HashMap::new();

        for y in 0..self.height + 1 {
            for x in 0..self.width + 1 {
                let mut enhanced_index = 0;
                let mut neighbours = MapKey::new(x as i32, y as i32).get_neighbours();
                neighbours.reverse();
                for (index, neighbour) in neighbours.iter().enumerate() {
                    if self.get_at(neighbour) == 1 {
                        enhanced_index += 2_u32.pow(index as u32);
                    }
                }
                new_map.insert(
                    MapKey::new((x + 1) as i32, (y + 1) as i32).get_key(),
                    if self
                        .enhancement_algorithm
                        .chars()
                        .nth(enhanced_index as usize)
                        .unwrap()
                        .to_string()
                        .trim()
                        == "#"
                    {
                        1
                    } else {
                        0
                    },
                );
            }
        }
        self.width += 2;
        self.height += 2;
        self.pixels = new_map.clone();
    }

    fn draw(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if self.get_at(&MapKey::new(x as i32, y as i32)) == 1 {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input1").expect("Something went wrong reading the file");
    let start = Instant::now();
    let mut v = contents.trim().split('\n');
    let enhancement_algorithm = v.next().unwrap();
    v.next();
    let mut image = Image::new(v.collect::<Vec<&str>>(), enhancement_algorithm);
    //println!("{:?}", image);

    image.draw();
    image.enhance();
    println!();
    image.draw();
    image.enhance();
    println!();
    image.draw();

    println!(
        "{:?}",
        image.pixels.into_iter().filter(|(_k, v)| *v > 0).count()
    );

    println!("took {:?}", start.elapsed());
}
