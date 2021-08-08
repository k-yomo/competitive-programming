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
        n: usize,
        mut a: [usize; n],
    }

    let mut a_with_i = a.iter().enumerate().collect::<Vec<(usize, &usize)>>();
    a_with_i.sort_by_key(|x| x.1);

    println!("{}", a_with_i[a.len() - 2].0 + 1)
}
