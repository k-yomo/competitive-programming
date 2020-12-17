#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut num_map = HashMap::new();
    let mut first = 0;
    let mut second = 0;
    for num in a {
        let count: &mut i64 = num_map.entry(num).or_insert(0);
        *count += 1;
        if *count < 2 {
            continue
        }
        if num == first || num == second {
            continue
        }
        if second < num {
            first = second;
            second = num;
        } else if first < num {
            first = num;
        }
    }
    println!("first: {}, second: {}", first, second);
    println!("{}", first * second);
}
