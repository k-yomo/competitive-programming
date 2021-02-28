#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        n: usize,
        mut red: [(usize, usize); n],
        mut blue: [(usize, usize); n],
    }

    red.sort_by_key(|x| x.1);
    blue.sort_by_key(|x| x.0);

    let mut match_count = 0;
    for (blue_x, blue_y) in blue {
        for (i, (red_x, red_y)) in red.iter().rev().enumerate() {
            if *red_x < blue_x && *red_y < blue_y {
                match_count += 1;
                red.remove(red.len() - 1 - i);
                break;
            }
        }
    }

    println!("{}", match_count);
}
