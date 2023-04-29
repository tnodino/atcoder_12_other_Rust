// https://atcoder.jp/contests/APG4b/tasks/APG4b_cb

use proconio::input;
use proconio::fastout;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f0(N: usize) -> i32 {
    return 1;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f1(N: usize, M: usize) -> i32 {
    let mut s = 0;
    for _ in 0..N {
        s += 1;
    }
    for _ in 0..M {
        s += 1;
    }
    return s;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f2(N: usize) -> i32 {
    let mut s = 0;
    for _ in 0..N {
        let mut t = N;
        let mut cnt = 0;
        while t > 0 {
            cnt += 1;
            t /= 2;
        }
        s += cnt;
    }
    return s;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f3(N: usize) -> i32 {
    let mut s = 0;
    for _ in 0..3 {
        for _ in 0..3 {
            s += 1;
        }
    }
    return s;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f4(N: usize) -> i32 {
    let mut s = 0;
    for i in 0..N {
        for j in 0..N {
            s += (i + j) as i32;
        }
    }
    return s;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn f5(N: usize, M: usize) -> i32 {
    let mut s = 0;
    for i in 0..N {
        for j in 0..M {
            s += (i + j) as i32;
        }
    }
    return s;
}

#[fastout]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut a0 = -1i32;
    let mut a1 = -1i32;
    let mut a2 = -1i32;
    let mut a3 = -1i32;
    let mut a4 = -1i32;
    let mut a5 = -1i32;
    a0 = f0(N);
    a1 = f1(N, M);
    a2 = f2(N);
    a3 = f3(N);
    //a4 = f4(N);
    a5 = f5(N, M);
    println!("f0: {}", a0);
    println!("f1: {}", a1);
    println!("f2: {}", a2);
    println!("f3: {}", a3);
    println!("f4: {}", a4);
    println!("f5: {}", a5);
}