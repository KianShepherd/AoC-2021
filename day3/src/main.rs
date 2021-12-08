use std::fs;

#[derive(Debug)]
struct Counter {
    one_count: i32,
    zero_count: i32,
}

enum BitCriteria {
    Most,
    Least,
}

fn filter_arr(nums: Vec<Vec<i32>>, index: usize, criteria: BitCriteria) -> Vec<Vec<i32>>{
    let mut count_ones = 0;
    let mut count_zeros = 0;
    let mut ones = vec![];
    let mut zeros = vec![];

    for num_arr in nums {
        if num_arr[index] == 1 {
            count_ones += 1;
            ones.push(num_arr);
        } else {
            count_zeros += 1;
            zeros.push(num_arr);
        }
    }

    match criteria {
        BitCriteria::Most => {
            if count_ones > count_zeros {
                ones
            } else if count_zeros > count_ones {
                zeros
            } else {
                ones
            }
        },
        BitCriteria::Least => {
            if count_ones > count_zeros {
                zeros
            } else if count_zeros > count_ones {
                ones
            } else {
                zeros
            }
        },
    }
}

fn num_arr_to_i32(nums: &Vec<i32>) -> i32 {
    let rev = {
        let mut r = nums.clone();
        r.reverse();
        r
    };

    let mut total = 0;
    for i in 0..rev.len() {
        if rev[i] == 1 {
            total += 2_i32.pow(i as u32);
        }
    }

    total
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split('\n').collect();
    let mut values = vec![];
    for line in v {
        let mut nums = vec![];
        for chr in line.chars() {
            if chr == '1' {
                nums.push(1);
            } else {
                nums.push(0);
            }
        }
        if nums.len() > 1 {
            values.push(nums);
        }
    }
    //println!("{:?}", values);

    let inner_len = values[0].len();
    let mut oxegen_rating = values.clone();
    let mut coo_rating = values.clone();
    for i in 0..inner_len {
        if oxegen_rating.len() != 1 {
            oxegen_rating = filter_arr(oxegen_rating, i, BitCriteria::Most);
        }
        if coo_rating.len() != 1 {
            coo_rating = filter_arr(coo_rating, i, BitCriteria::Least);
        }
    }
    //let oxegen_rating = oxegen_rating[0].clone();
    //let coo_rating = coo_rating[0].clone();
    //println!("{:?}", oxegen_rating);
    //println!("{:?}", coo_rating);
    println!("{:?}", num_arr_to_i32(&oxegen_rating[0]) * num_arr_to_i32(&coo_rating[0]));
}
