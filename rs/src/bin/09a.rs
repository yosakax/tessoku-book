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
        H:usize, W:usize, N:usize,
        ABCD:[(usize, usize, usize, usize); N]
    }
    let mut mat = vec![vec![0; W + 2]; H + 2];
    for &(a, b, c, d) in ABCD.iter() {
        mat[a][b] += 1;
        mat[a][d + 1] -= 1;
        mat[c + 1][b] -= 1;
        mat[c + 1][d + 1] += 1;
    }
    for i in 0..H + 2 {
        for j in 0..W + 2 {
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
    for i in 1..H + 1 {
        let mut tmp = vec![];
        for j in 1..W + 1 {
            tmp.push(mat[i][j]);
        }
        println!("{}", tmp.iter().join(" "));
    }
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

