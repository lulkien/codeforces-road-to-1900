#![allow(unused)]

use std::io::{self, BufRead};

macro_rules! take_var {
    ($t:ty) => {{
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        line.split_whitespace()
            .next()
            .unwrap()
            .parse::<$t>()
            .unwrap()
    }};
}

macro_rules! take_vec {
    ($t:ty) => {{
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|s| s.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

macro_rules! take_tuple {
    ( $( $t:ty ),+ $(,)? ) => {{
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let mut iter = line.split_whitespace();

        (
            $(
                iter.next().unwrap().parse::<$t>().unwrap()
            ),*
        )
    }};
}

fn solve() {
    // =========================== YOUR  CODE HERE ===========================
}

fn main() {
    let mut matrix: Vec<Vec<i8>> = Vec::new();

    let (mut row_idx, mut col_idx): (i64, i64) = (0, 0);

    for row in 0..5 {
        let row_data = take_vec!(i8);
        for (idx, ele) in row_data.iter().enumerate() {
            if (*ele == 1) {
                col_idx = idx as i64;
                row_idx = row;
                break;
            }
        }
    }

    println!("{}", (2 - col_idx).abs() + (2 - row_idx).abs());
}
