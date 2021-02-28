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
        n: usize,
        a: [usize; n],
    }

    let mut max_count = 0_usize;
    let mut num_map = HashMap::new();
    for i in a.iter() {
        num_map.insert(i, true);
    }

    for key in num_map.keys() {
        let (mut l, mut r, mut x) = (0, 0, key);
        while l < n {
            while r < n && x <= &&a[r] {
                r += 1;
            }
            max_count = std::cmp::max(max_count, (r - l) * **x);
            if l == r {
                r += 1;
            }
            l += 1;
        }
    }

    println!("{}", max_count);
}
