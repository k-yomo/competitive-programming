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
        n: usize,
        s: Chars,
        t: Chars,
    }

    let mut queue = VecDeque::new();
    let mut count = 0;
    for i in 0..n {
        if s[i] == t[i] {
            continue;
        }
        if !queue.is_empty() && s[i] == '1' {
            count += i - queue.pop_front().unwrap();
        } else {
            queue.push_back(i);
        }
    }

    if queue.is_empty() {
        println!("{}", count)
    } else {
        println!("-1")
    }
}
