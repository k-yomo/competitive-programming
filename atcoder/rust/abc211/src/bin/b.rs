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

fn main() {
    input! {
        s: [String; 4],
    }

    let uniq: HashSet<String> = s.into_iter().collect();
    println!("{}", if uniq.len() == 4 { "Yes" } else { "No" })
}
