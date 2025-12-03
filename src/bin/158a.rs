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

fn solve(n: usize, k: usize) {
    let scores = take_vec!(i32);
    let pass_score = scores[k - 1];

    let result = scores
        .into_iter()
        .filter(|&sc| sc > 0 && sc >= pass_score)
        .count();

    println!("{result}");
}

fn main() {
    let (n, k): (usize, usize) = take_tuple!(usize, usize);

    solve(n, k);
}
