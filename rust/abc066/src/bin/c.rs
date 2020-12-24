#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = VecDeque::with_capacity(n);
    for (i, &num) in a.iter().enumerate() {
        if (n-1) % 2 == i % 2 {
            ans.push_front(num.to_string())
        } else {
            ans.push_back(num.to_string())
        }
    }

    println!("{}", ans.into_iter().collect::<Vec<_>>().join(" "));
}
