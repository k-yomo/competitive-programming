#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        mut n: i64,
        ngs: [i64; 3],
    }

    if ngs.iter().any(|ng| n == *ng) {
        println!("NO");
        return
    }

    let mut count = 0;
    while n > 0 {
        let mut can_subtract = false;
        for i in (1..4).rev() {
            if n - i != ngs[0] && n - i != ngs[1] && n - i != ngs[2] {
                n -= i;
                count += 1;
                can_subtract = true;
                break
            }
        }
        if !can_subtract {
            println!("NO");
            return
        }
    }
    if count > 100 {
        println!("NO");
        return
    }

    println!("YES");
}
