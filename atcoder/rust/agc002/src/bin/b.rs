#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize, m: usize,
        xy: [(usize, usize); m]
    }

    let mut boxes = vec![1; n];
    let mut red_boxes = vec![false; n];
    red_boxes[0] = true;
    let mut cur_red = 0;
    for (x, y) in xy {
        if red_boxes[x - 1] {
            red_boxes[y - 1] = true
        }
        if boxes[x - 1] == 1 {
            red_boxes[x - 1] = false
        }
        boxes[x - 1] -= 1;
        boxes[y - 1] += 1;
    }

    println!("{}", red_boxes.iter().filter(|x| **x).count())
}
