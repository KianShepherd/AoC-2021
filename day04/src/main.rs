use std::fs;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
struct BingoNumber {
    num: i32,
    selected: bool,
}
#[derive(Debug, Clone)]
struct Board {
    nums: Vec<BingoNumber>,
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

impl Board {
    fn new(nums: Vec<BingoNumber>, rows: Vec<Vec<usize>>, cols: Vec<Vec<usize>>) -> Self {
        Self { nums, rows, cols }
    }

    fn play_bingo(&mut self, bingo_num: i32) {
        for i in 0..self.nums.len() {
            if self.nums[i].num == bingo_num {
                self.nums[i].selected = true;
                break;
            }
        }
    }

    fn is_done(&self) -> bool {
        for row in &self.rows {
            let mut all_selected = true;
            for index in row {
                if !self.nums[*index].selected {
                    all_selected = false;
                    break;
                }
            }
            if all_selected {
                return true;
            }
        }
        for col in &self.cols {
            let mut all_selected = true;
            for index in col {
                if !self.nums[*index].selected {
                    all_selected = false;
                    break;
                }
            }
            if all_selected {
                return true;
            }
        }
        false
    }

    fn sum_not_selected(&self) -> i32 {
        let mut total = 0;
        for num in &self.nums {
            if !num.selected {
                total += num.num;
            }
        }
        total
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let v: Vec<&str> = contents.split('\n').collect();
    let bingo_nums = v[0].split(",").collect::<Vec<&str>>();
    let mut rows = vec![];
    let mut cols = vec![];
    for i in 0..5 {
        let mut row = vec![];
        let mut col = vec![];
        for j in 0..5 {
            row.push(j + (i * 5));
            col.push((j * 5) + i);
        }
        rows.push(row);
        cols.push(col);
    }
    //println!("{:?}", bingo_nums);

    let mut bingo_boards = vec![];
    let mut nums = vec![];
    for vi in v.iter().skip(2) {
        if vi.is_empty() {
            bingo_boards.push(Board::new(nums, rows.clone(), cols.clone()));
            nums = vec![];
        } else {
            for num in vi.split(" ") {
                if num.is_empty() {
                    continue;
                }
                nums.push(BingoNumber {
                    num: num.parse::<i32>().unwrap(),
                    selected: false,
                });
            }
        }
    }
    let mut i = 0;
    let mut first_board_done = false;
    let mut first_done = Duration::new(0, 0);
    let mut first_answer = 0;

    loop {
        let bingo_num = bingo_nums[i].parse::<i32>().unwrap();
        let mut temp_boards = vec![];
        let boards_length = bingo_boards.len();
        for board in bingo_boards.iter_mut() {
            board.play_bingo(bingo_num);
            if !first_board_done && board.is_done() {
                first_board_done = true;
                first_done = start.elapsed();
                first_answer = board.sum_not_selected() * bingo_num;
            }
            if boards_length == 1 && board.is_done() {
                println!("{:?} : took {:?}", first_answer, first_done);
                println!("{:?} : took {:?}", bingo_boards[0].sum_not_selected() * bingo_num, start.elapsed());
                return;
            }
            if !board.is_done() {
                temp_boards.push(board.clone());
            }
        }
        bingo_boards = temp_boards;
        i += 1;
    }
}
