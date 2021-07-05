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

    for i in 0..=n {
        let mut ok = true;
        for j in i..n {
            if s[j] != t[j - i] {
                ok = false
            }
        }
        if ok {
            return println!("{}", n + i);
        }
    }
}
