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
use regex::Regex;
use superslice::*;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.replace('\n', "");
    Regex::new(r"@[a-z]+")
        .unwrap()
        .captures_iter(&s)
        .map(|caps| caps.get(0).unwrap().as_str().replace("@", ""))
        .unique()
        .sorted()
        .for_each(|user| println!("{}", user))
}
