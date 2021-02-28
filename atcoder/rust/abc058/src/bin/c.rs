#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use proconio::marker::Chars;

fn main() { 
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut letter_count_maps = vec![];
    for chars in s.iter() {
        let mut letter_count_map = HashMap::new();
        for c in chars {
            let count = letter_count_map.entry(c).or_insert(0);
            *count += 1;
        }
        letter_count_maps.push(letter_count_map);
    }

    let mut ans = vec![];
    for (&&c, &count) in letter_count_maps[0].iter() {
        let mut min_count = count;
        for letter_count_map in letter_count_maps[1..].iter() {
            min_count = min(min_count, *letter_count_map.get(&c).unwrap_or(&0));
        }
        (0..min_count).for_each(|| { ans.push(c) });
    }
    ans.sort();

    if ans.len() > 0 {
        println!("{}", ans.iter().collect::<String>());
    }
}
