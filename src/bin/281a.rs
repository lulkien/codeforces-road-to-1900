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
    let mut s = take_var!(String);

    let first = s.chars().next().unwrap().to_uppercase().to_string();
    let rest = &s[1..];

    println!("{}{}", first, rest)
}
