#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut si = 0;
    let mut ti = 0;
    loop {
        if si == s.len() || ti == t.len() {
            if si != s.len() || ti != t.len() {
                return println!("No")
            }
            break
        }
        if s[si] != t[ti] {
            return println!("No")
        }

        let mut si_count = 0;
        while si < s.len() - 1 && s[si] == s[si+1] {
            si += 1;
            si_count += 1;
        }
        let mut ti_count = 0;
        while ti < t.len() -1 && t[ti] == t[ti+1] {
            ti += 1;
            ti_count += 1;
        }
        if (si_count == 0 && ti_count > 0) || si_count > ti_count {
            return println!("No")
        }
        si += 1;
        ti += 1;
    }

    println!("Yes")
}
