#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max_len = 0;
    let (mut l, mut r) = (0, 0);
    let mut cur_taste_map: HashMap<usize, bool> = HashMap::new();
    while l < n {
        while r < n && !*cur_taste_map.entry(a[r]).or_default() {
            cur_taste_map.insert(a[r], true);
            r += 1;
        }
        max_len = std::cmp::max(max_len, r - l);
        if l == r {
            r += 1;
        } else {
            cur_taste_map.remove(&a[l]);
        }
        l += 1;
    }

    println!("{}", max_len);
}
