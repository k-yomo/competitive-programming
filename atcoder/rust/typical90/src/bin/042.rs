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
use whiteread::parse_string;

fn main() {
    input! {
        k: usize,
    }

    const m: usize = 1000000007;

    // i桁目までを使って、合計をkにする組み合わせ数
    let mut dp = vec![vec![]];
    // 合計がmになる組み合わせを列挙
    // xの倍数か判定
    let mut count = 0;
    let mut digit = 1;
    while k / digit <= 9 {
        digit += 1;
    }

    println!("{}", count)
}

fn digit_sum(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    return digit_sum(n / 10) + n % 10;
}
