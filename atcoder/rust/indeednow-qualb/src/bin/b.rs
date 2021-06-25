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
        mut s: Chars,
        t: Chars,
    }

    if s.len() != t.len() {
        return println!("{}", -1);
    }

    for i in 0..s.len() + 1 {
        let mut ok = true;
        for j in 0..s.len() {
            if s[j] != t[j] {
                ok = false;
            }
        }
        if ok {
            return println!("{}", i);
        }
        let last = s.pop().unwrap();
        s.insert(0, last);
    }

    println!("{}", -1);
}
