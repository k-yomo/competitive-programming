#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
        d: usize,
        g: usize,
        pc: [(usize, usize); d],
    }

    let mut min_problem_count = 1000;
    for bit_flag in 0..(1 << d) {
        let mut problem_count = 0;
        let mut score = 0;
        for i in 0..d {
            if bit_flag & (1 << i) != 0 {
                problem_count += pc[i].0;
                score += (i+1) * 100 * pc[i].0 + pc[i].1;
            }
        }
        if score < g {
            for i in (0..d).rev() {
                if bit_flag & (1 << i) != 0 {
                    continue
                }
                for _ in 0..pc[i].0 {
                    if score >= g {
                        break
                    }
                    problem_count += 1;
                    score += (i+1) * 100;
                }
            }
        }
        min_problem_count = std::cmp::min(problem_count, min_problem_count);
    }
    println!("{}", min_problem_count);
}
