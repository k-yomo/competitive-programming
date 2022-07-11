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
        ymd: String,
    }

    let nums = ymd.split("/").
        collect::<Vec<&str>>().
        iter().
        map(|i| i.to_string().parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    let mut year = nums[0];
    let mut month = nums[1];
    let mut day = nums[2];

    loop {
        if year % month == 0 && year / month % day == 0 {
            return println!("{}/{:02}/{:02}", year, month, day);
        }
        let is_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        if day == 31 || (month == 2 && (day == if is_leap_year { 29 } else { 28 })) || (vec![4, 6, 9, 11].contains(&month) && day == 30) {
            day = 1;
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
        } else {
            day += 1;
        };
    }
}
