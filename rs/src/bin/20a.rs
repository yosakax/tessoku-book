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
        S:Chars,
        T:Chars
    }
    let N = S.len();
    let M = T.len();
    let mut dp = vec![vec![0; M + 1]; N + 1];
    for i in 0..N {
        for j in 0..M {
            if i > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            }
            dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
            if S[i] == T[j] {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }
    let mut ans = 0;
    for i in 0..N + 1 {
        for j in 0..M + 1 {
            ans = ans.max(dp[i][j]);
        }
    }
    println!("{ans}");
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

