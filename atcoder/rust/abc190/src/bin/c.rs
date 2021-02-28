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
        n: usize, m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    }


    let mut max_count = 0;
    for bit_flag in 0..(1 << k) {
        let mut plate_balls = HashMap::new();
        for i in 0..k {
            if bit_flag & 1 << i != 0 {
                let count: &mut usize = plate_balls.entry(cd[i].0).or_insert(0);
                *count += 1;
            } else {
                let count: &mut usize = plate_balls.entry(cd[i].1).or_insert(0);
                *count += 1;
            }
        }
        let mut count = 0;
        for (a, b) in ab.iter() {
            if  *plate_balls.entry(*a).or_insert(0) > 0 && *plate_balls.entry(*b).or_insert(0) > 0 {
                count += 1;
            }
        }
        max_count = std::cmp::max(max_count, count);
    }

    println!("{}", max_count);
}
