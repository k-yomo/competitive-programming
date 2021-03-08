#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        mut deg: usize, mut dis: usize,
    }
    let degs = vec![
        "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW",
        "NNW",
    ];
    let w_powers = vec![
        24, 154, 334, 544, 794, 1074, 1384, 1714, 2074, 2444, 2844, 3264, 120000,
    ];

    if dis * 10 <= 24 * 6 {
        return println!("C 0");
    }

    deg *= 10;
    let s = 1125;
    let mut deg_mark = "N";
    if s <= deg && deg < 34875 {
        for i in 0..degs.len() {
            if deg < s + 2250 * (i + 1) {
                deg_mark = degs[i];
                break;
            }
        }
    }

    print!("{}", deg_mark);

    for (i, wp) in w_powers.iter().enumerate() {
        if dis * 10 <= *wp * 6 {
            return println!(" {}", i);
        }
    }
}
