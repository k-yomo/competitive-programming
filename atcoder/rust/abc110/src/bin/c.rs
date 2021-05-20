#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut char_map = HashMap::new();
    let mut used_char_map = HashMap::new();
    for i in 0..s.len() {
        match char_map.get(&s[i]) {
            Some(&c) => {
                if c != t[i] {
                    return println!("No");
                }
            }
            None => {
                if used_char_map.contains_key(&t[i]) {
                    return println!("No");
                } else {
                    char_map.insert(s[i], t[i]);
                    used_char_map.insert(t[i], true);
                }
            }
        }
    }

    println!("Yes")
}
