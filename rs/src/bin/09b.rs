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
        ABCD:[(usize,usize,usize,usize); N]
    }
    let M = 1510;
    let mut mat = vec![vec![0; M]; M];
    for &(a, b, c, d) in ABCD.iter() {
        mat[a][b] += 1;
        mat[a][d] -= 1;
        mat[c][b] -= 1;
        mat[c][d] += 1;
    }
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if i > 0 {
                mat[i][j] += mat[i - 1][j];
            }
            if j > 0 {
                mat[i][j] += mat[i][j - 1];
            }
            if i > 0 && j > 0 {
                mat[i][j] -= mat[i - 1][j - 1];
            }
        }
    }
    let mut ans = 0;
    for i in 0..mat.len() {
        for j in 0..mat.len() {
            if mat[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

