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
        A:[usize; N]
    }
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    let n1 = N / 2;
    for bit in 0..1 << n1 {
        let mut tmp = 0;
        for i in 0..n1 {
            if bit >> i & 1 == 1 {
                tmp += A[i];
            }
        }
        arr1.push(tmp);
    }
    for bit in 0..1 << (N - n1) {
        let mut tmp = 0;
        for i in 0..N - n1 {
            if bit >> i & 1 == 1 {
                tmp += A[i + n1];
            }
        }
        arr2.push(tmp);
    }

    arr1.sort();
    arr2.sort();
    eprintln!("{:?}", arr1);
    eprintln!("{:?}", arr2);
    for &a in arr1.iter() {
        let mut left = 0;
        let mut right = arr2.len();
        while right - left > 1 {
            let mid = (left + right) / 2;
            if a + arr2[mid] > K {
                right = mid;
            } else {
                left = mid;
            }
            // eprintln!("left = {left}, right ={right}");
        }
        if a + arr2[left] == K {
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

