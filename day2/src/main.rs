use std::fs;

struct SubLocation {
    forward: i32,
    depth: i32,
    aim: i32,
}

impl SubLocation {
    pub fn update(&mut self, command: &str) {
        if command == "" { return; }

        let mut split_command= command.split(" ");
        let command = split_command.next().unwrap();
        let num = split_command.next().unwrap().to_string().parse::<i32>().unwrap();
        match command {
            "forward" => { 
                self.forward += num;
                self.depth += num * self.aim;
             },
            "down" => { self.aim += num; },
            "up" => { self.aim -= num; },
            &_ => (),
        };
    }

    pub fn final_answer(&self) {
        println!("{:?}", &self.forward * &self.depth);
    }
}

fn main() {
    let contents = fs::read_to_string("input")
       .expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split("\n").collect();
    let mut sub = SubLocation {
        forward: 0,
        depth: 0,
        aim: 0,
    };

    for command in v {
        sub.update(command);
    }

    sub.final_answer();
}