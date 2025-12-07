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
    let mut n = take_var!(usize);

    let mut step: usize = 0;

    if n % 5 == 0 {
        step += n / 5;
        println!("{step}");
        return;
    }

    step += n / 5;
    n %= 5;

    if n % 4 == 0 {
        step += n / 4;
        println!("{step}");
        return;
    }

    step += n / 4;
    n %= 4;

    if n % 3 == 0 {
        step += n / 3;
        println!("{step}");
        return;
    }

    step += n / 3;
    n %= 3;

    if n % 2 == 0 {
        step += n / 2;
        println!("{step}");
        return;
    }

    step += n / 2;
    n %= 2;

    step += n;
    println!("{step}")
}
