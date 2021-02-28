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
        k: usize,
        s: Chars,
        t: Chars,
    }


    let mut s_nums = vec![0; 10];
    let mut t_nums = vec![0; 10];
    let mut rem_nums = vec![k; 10];
    for c in s[0..4].iter() {
        let i = c.to_string().parse::<usize>().unwrap() - 1;
        s_nums[i] += 1;
        rem_nums[i] -= 1;
    }
    for c in t[0..4].iter() {
        let i = c.to_string().parse::<usize>().unwrap() - 1;
        t_nums[i] += 1;
        rem_nums[i] -= 1;
    }

    let mut won_count = 0;
    for i in 0..9 {
        if rem_nums[i] == 0 {
            continue;
        }

        s_nums[i] += 1;
        for j in 0..9 {
            if rem_nums[j] == 0 {
                continue;
            }
            t_nums[j] += 1;

            if calc_score(&s_nums) > calc_score(&t_nums) {
                won_count += rem_nums[i] * (rem_nums[j] - if i == j { 1 } else { 0 })
            }
            t_nums[j] -= 1;
        }
        s_nums[i] -= 1;
    }

    let rem = (9 * k - 8) as f64;
    println!("{}", won_count as f64 / rem / (rem - 1.0));
}

fn calc_score(nums: &Vec<usize>) -> usize {
    return nums.iter().enumerate().map(|(i, count)| (i + 1) * 10_usize.pow(*count as u32)).sum::<usize>();
}