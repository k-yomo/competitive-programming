#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use ndarray::StrideShape;
use proconio::marker::*;
use proconio::*;
use regex::Regex;
use superslice::*;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.replace('\n', "");
    let mut s = s.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
    input! {
        n: usize,
        t: [Chars; n],
    };

    for i in 0..n {
        for j in 0..s.len() {
            if s[j].len() == t[i].len() {
                if (0..s[j].len())
                    .all(|k| t[i][k] == '*' || s[j].chars().nth(k).unwrap() == t[i][k])
                {
                    s[j] = (0..t[i].len())
                        .map(|_| '*')
                        .collect::<std::string::String>();
                }
            }
        }
    }

    println!("{}", s.join(" "));
}
