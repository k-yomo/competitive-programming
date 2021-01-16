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
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut num_map = HashMap::new();
    for num in a {
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut nums = num_map
        .into_iter()
        .map(|(num, count)| (num, count))
        .collect::<Vec<(usize, usize)>>();
    nums.sort_by_key(|x| x.0);

    if nums[0].0 != 0 {
        println!("0");
        return;
    }

    let mut max_score = std::cmp::min(k, nums[0].1);
    let mut rest_count = std::cmp::min(k, nums[0].1);
    for (i, (num, count)) in nums[1..].iter().enumerate() {
        if *num != nums[i].0 + 1 {
            break;
        }
        rest_count = std::cmp::min(k, std::cmp::min(rest_count, *count));
        max_score += rest_count;
    }
    println!("{}", max_score);
}
