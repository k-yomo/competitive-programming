#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use whiteread::parse_string;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() <= 2 {
        return if parse_string::<usize>(&s.iter().collect::<String>()).unwrap() % 8 == 0 ||
            parse_string::<usize>(&s.iter().rev().collect::<String>()).unwrap() % 8 == 0 {
            println!("Yes")
        } else {
            println!("No")
        };
    }

    let mut num_counts = vec![0; 10];
    for c in s {
        num_counts[(c as u8 - '0' as u8) as usize] += 1;
    }

    for i in (112..1000).step_by(8) {
        let a = i / 100;
        let b = (i / 10) % 10;
        let c = i % 10;
        if a == 0 || b == 0 || c == 0 {
            continue;
        }
        num_counts[a] -= 1;
        num_counts[b] -= 1;
        num_counts[c] -= 1;
        if num_counts[a] >= 0 && num_counts[b] >= 0 && num_counts[c] >= 0 {
            return println!("Yes");
        }
        num_counts[a] += 1;
        num_counts[b] += 1;
        num_counts[c] += 1;
    }
    println!("No");
}
