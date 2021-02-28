#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    a.sort();
    let mut num_map = HashMap::new();
    let mut min_len = 0;
    let mut max_len = 0;
    for num in a {
        let count: &mut i64 = num_map.entry(num).or_insert(0);
        *count += 1;
        if *count < 2 {
            continue
        }
        if *count < 4 && (num == min_len || num == max_len) {
            continue
        }
        if max_len < num {
            min_len = max_len;
            max_len = num;
        } else if min_len < num {
            min_len = num;
        }
    }
    if min_len == 0 || max_len == 0 {
        println!("0");
        return
    }
    println!("{}", min_len * max_len);
}
