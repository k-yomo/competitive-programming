#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        mut n: usize,
    }

    // 1
    // 6 * n, 36, 216, 1296, 7776, 466656
    // 9 * n, 81, 729, 6561, 59409

    // 127 = 81, 36, 9, 1
    let power_of_6: Vec<usize> = vec![6, 36, 216, 1296, 7776, 46656].into_iter().take_while(|x| *x <= n).collect();
    let power_of_9: Vec<usize> = vec![9, 81, 729, 6561, 59409].into_iter().take_while(|x| *x <= n).collect();
    let mut count = 0;
    // while n > 0 {
    // let p6 = power_of_6.lower_bound(&n);
    // let p9 = power_of_6.lower_bound(&n);
    println!("{}", power_of_9.lower_bound(&n));
    // }

    println!("{}", count);
}
