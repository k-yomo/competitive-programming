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
        s: Chars,
    }

    let mut char_counts = vec![0; 3];
    for c in s {
        let i = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => 0,
        };
        char_counts[i] += 1;
    }

    if char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap() <= 1 {
        println!("YES")
    } else {
        println!("NO")
    }
}
