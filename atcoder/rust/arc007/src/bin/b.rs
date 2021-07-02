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
        n: usize, m: usize,
        disk: [usize; m],
    }

    let mut cds = (1..=n).map(|i| i).collect::<Vec<usize>>();
    let mut cd_case_map: HashMap<usize, usize> = HashMap::new();
    for i in 1..=n {
        cd_case_map.insert(i, i - 1);
    }
    let mut cur_cd = 0;
    for i in disk {
        if i == cur_cd {
            continue;
        }
        let pos = *cd_case_map.get(&i).unwrap();
        let temp = cds[pos];
        cd_case_map.insert(cur_cd, pos);
        cds[pos] = cur_cd;
        cur_cd = temp;
    }

    cds.iter().for_each(|cd| println!("{}", cd))
}
