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
       s: String
    }
    if &s[..7] == "keyence"
        || (&s[..1] == "k" && &s[s.len() - 6..] == "eyence")
        || (&s[..2] == "ke" && &s[s.len() - 5..] == "yence")
        || (&s[..3] == "key" && &s[s.len() - 4..] == "ence")
        || (&s[..4] == "keye" && &s[s.len() - 3..] == "nce")
        || (&s[..5] == "keyen" && &s[s.len() - 2..] == "ce")
        || (&s[..6] == "keyenc" && &s[s.len() - 1..] == "e")
        || &s[s.len() - 7..] == "keyence"
    {
        println!("YES")
    } else {
        println!("NO")
    }
}
