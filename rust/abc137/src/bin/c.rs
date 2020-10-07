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

    let mut sorted_word_map: HashMap<String, usize> = HashMap::new();
    for word in s {
        let mut chars: Vec<_> = word.chars().collect();
        chars.sort();
        let mut count = sorted_word_map.entry(chars.into_iter().collect()).or_default();
        *count += 1;
    }

    let mut anagram_count = 0;
    for (_, v) in sorted_word_map.iter() {
        anagram_count += v * (v-1) / 2
    }

    println!("{}", anagram_count);
}
