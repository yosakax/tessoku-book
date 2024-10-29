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
        N:usize, K:usize,
        A:[usize; N],
        B:[usize; N],
        C:[usize; N],
        D:[usize; N]
    }
    let mut AB = vec![];
    let mut CD = vec![];
    for i in 0..N {
        for j in 0..N {
            AB.push(A[i] + B[j]);
            CD.push(C[i] + D[j]);
        }
    }
    AB.sort();
    CD.sort();
    // eprintln!("{:?}", AB);
    // eprintln!("{:?}", CD);
    for &a in AB.iter() {
        let mut left = 0;
        let mut right = CD.len();
        while right - left > 1 {
            let mid = (left + right) / 2;
            if a + CD[mid] > K {
                right = mid;
            } else {
                left = mid;
            }
        }
        if a + CD[left] == K {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

