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
        n: usize,
    }

    if n <= 2 {
        return println!("0");
    }

    let mut fib = vec![0; n];
    fib[2] = 1;
    for i in 3..n {
        fib[i] = (fib[i-3] + fib[i-2] + fib[i-1]) % 10007
    }

    println!("{}", fib[n-1]);
}
