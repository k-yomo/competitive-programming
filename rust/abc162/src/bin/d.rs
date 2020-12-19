#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let r = s.iter().filter(|&&c| c == 'R').count();
    let g = s.iter().filter(|&&c| c == 'G').count();
    let b = s.iter().filter(|&&c| c == 'B').count();
    let all = r * g * b;

    let mut sub = 0;
    for i in 0..n {
        for j in i+1..n {
            let k = j * 2 - i;
            if k >= n || s[i] == s[j] || s[i] == s[k] || s[j] == s[k] {
                continue
            }
            sub += 1;
        }
    }
    println!("{:?}", all - sub);
}
