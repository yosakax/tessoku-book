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
        A:[usize; N],
        D:usize,
        LR:[(usize, usize); D]
    }
    let mut a1 = vec![0];
    let mut a2 = vec![0];
    for i in 0..N {
        a1.push(a1[i].max(A[i]));
    }
    for i in (0..N).rev() {
        a2.push(a2[N - i - 1].max(A[i]));
    }
    let mut ans = vec![];
    a2.reverse();
    for &(l, r) in LR.iter() {
        ans.push(a1[l - 1].max(a2[r]));
    }
    println!("{}", ans.iter().join("\n"));
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

