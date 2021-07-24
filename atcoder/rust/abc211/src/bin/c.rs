#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

const Mod: u128 = 1000000007;

fn main() {
    input! {
        s: Chars
    }

    let mut counts = vec![0; 8];
    for c in s {
        match c {
            'c' => counts[0] = (counts[0] + 1) % Mod,
            'h' => counts[1] = (counts[1] + counts[0]) % Mod,
            'o' => counts[2] = (counts[2] + counts[1]) % Mod,
            'k' => counts[3] = (counts[3] + counts[2]) % Mod,
            'u' => counts[4] = (counts[4] + counts[3]) % Mod,
            'd' => counts[5] = (counts[5] + counts[4]) % Mod,
            'a' => counts[6] = (counts[6] + counts[5]) % Mod,
            'i' => counts[7] = (counts[7] + counts[6]) % Mod,
            _ => {}
        }
    }

    println!("{}", counts[7])
}
