#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    }

    let mut wordMap: HashMap<&String, i64> = HashMap::new();
    for word in s.iter() {
        let count: &mut i64 = wordMap.entry(word).or_insert(0);
        *count += 1;
    }
    for word in t.iter() {
        let count: &mut i64 = wordMap.entry(word).or_insert(0);
        *count -= 1;
    }
    
    let max = wordMap.values().max().unwrap();
    println!("{}", if max > &0_i64 { max } else { &0_i64 });
}
