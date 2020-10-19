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
        s: [String; n],
    }

    let mut judge_map: HashMap<String, usize> = HashMap::new();
    for judge in s {
        *judge_map.entry(judge).or_insert(0) += 1;
    }

    println!("AC x {}", judge_map.entry("AC".to_string()).or_default());
    println!("WA x {}", judge_map.entry("WA".to_string()).or_default());
    println!("TLE x {}", judge_map.entry("TLE".to_string()).or_default());
    println!("RE x {}", judge_map.entry("RE".to_string()).or_default());
}
