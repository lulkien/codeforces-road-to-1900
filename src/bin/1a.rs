#![allow(unused)]

use std::{io::{self, BufRead}};

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
    let (n, m, a) = take_tuple!(u64, u64, u64);
    
    let n_need = (n as f64 / a as f64).ceil() as u64;
    let m_need = (m as f64 / a as f64).ceil() as u64;

    let needed = m_need * n_need;

    println!("{needed}")
}
