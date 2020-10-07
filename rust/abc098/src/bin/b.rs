#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
 
 
fn main() { 
    input! {
        n: usize,
        s: String,
    }

    let mut max = 0;
    for i in 1..n {
        let chars = s.chars().collect::<Vec<_>>();
        let (a,b) = chars.split_at(i);
        let mut a_char: HashMap<char, bool> = HashMap::new();
        for c in a {
            a_char.insert(*c, true);
        }
        let uniq_b: HashSet<_> = b.into_iter().collect();
        let mut count = 0;
        for c in uniq_b {
            if *a_char.entry(*c).or_default() {
                count += 1;
            }
        }
        if max < count {
            max = count;
        }
    }

    println!("{}", max);
}
