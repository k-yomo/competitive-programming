#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }

    for y in 0..h {
        for x in 0..w {
            if s[y][x] != '#'
                || y > 0 && s[y - 1][x] == '#'
                || y < h - 1 && s[y + 1][x] == '#'
                || x > 0 && s[y][x - 1] == '#'
                || x < h - 1 && s[y][x + 1] == '#'
            {
                continue;
            }
            println!("No");
            return;
        }
    }

    println!("Yes");
}
