#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::Itertools;
 
fn main() { 
    input! {
        n: usize,
        mut l: [usize; n],
    }
    l.sort();

    let mut triangle_count = 0;
    for s_i in 0..n - 1 {
        for m_i in s_i + 1..n {
            for l_i in (m_i + 1)..n {
                if l[s_i] + l[m_i] > l[l_i] {
                    triangle_count += 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", triangle_count);
}
