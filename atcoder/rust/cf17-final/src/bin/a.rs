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
    }

    if s.len() > 9 {
        return println!("NO");
    }

    let mut akihabara = String::from("AKIHABARA");
    let mut i = 0;
    for c in akihabara.chars() {
        if i < s.len() && c == s[i] {
            i += 1;
        } else {
            if c == 'A' {
                continue;
            }
            return println!("NO");
        }
    }
    println!("YES")
}
