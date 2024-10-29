#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
use ac_library::*;
use amplify::confinement::Collection;
use itertools::{CombinationsWithReplacement, Itertools};
use lazy_static::lazy_static;
use libm::{ceil, log};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_bigint::BigInt;
use num_integer::{div_ceil, Integer};
use proconio::marker::{Chars, Usize1};
use proconio::{input, source::line::LineSource};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::From;
use std::f64::consts::PI;
use std::hash::Hash;
use std::mem::swap;
use std::ops::Index;
use std::ops::Mul;
use std::{io::*, string};

fn solve() {
    #[rustfmt::skip]
    input! {
        N:usize,
        A:[usize; N]
    }
    let mut B = vec![];
    let mut used = HashSet::new();
    for &a in A.iter() {
        if used.contains(&a) {
            continue;
        }
        used.insert(a);
        B.push((a, 0));
    }
    B.sort_by_key(|a| a.0);
    for i in 0..B.len() {
        B[i].1 = i + 1;
    }
    let mut ans = vec![];
    for &a in A.iter() {
        let mut left = 0;
        let mut right = B.len();
        while right - left > 1 {
            let mid = (left + right) / 2;
            if B[mid].0 > a {
                right = mid;
            } else {
                left = mid;
            }
        }
        ans.push(B[left].1);
    }
    println!("{}", ans.iter().join(" "));
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

