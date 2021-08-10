#![allow(unused_imports)]

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
    }

    if n == 1 {
        return println!("Not Prime");
    }

    if is_prime(n) || (n % 10 % 2 != 0 && n % 10 != 5 && digit_sum(n) % 3 != 0) {
        println!("Prime")
    } else {
        println!("Not Prime")
    }
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1
    }
    return true;
}

fn digit_sum(mut num: usize) -> usize {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
