#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    if t.len() > s.len() {
        println!("UNRESTORABLE");
        return;
    }

    let mut is_replaced = false;
    let diff = s.len() - (t.len() - 1);
    for i in (0..diff).rev() {
        if (i..(i + t.len())).enumerate().all(|(i, j)| s[j] == '?' || s[j] == t[i]) {
            (i..(i + t.len())).enumerate().for_each(|(i, j)| s[j] = t[i]);
            is_replaced = true;
            break;
        }
    }
    if !is_replaced {
        println!("UNRESTORABLE");
        return;
    }
    println!("{}", s.iter().map(|&c| if c == '?' { 'a' } else { c }).collect::<String>());
}
