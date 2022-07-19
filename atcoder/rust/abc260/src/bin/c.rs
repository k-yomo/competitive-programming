#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: usize, x: usize, y: usize,
    }

    let mut level1_brew_count = 0;
    let mut queue = VecDeque::new();
    queue.push_back((0, n, 1));
    loop {
        if queue.is_empty() {
            break
        }
        let (color, level, count) = queue.pop_front().unwrap();
        if color == 0 {
            if level > 1 {
                queue.push_back((0, level - 1, count));
                queue.push_back((1, level, x * count));
            }
        } else {
            if level > 1 {
                queue.push_back((1, level - 1, y * count));
                queue.push_back((0, level - 1, count));
            } else {
                level1_brew_count += count;
            }
        }
    }

    println!("{}", level1_brew_count)
}
