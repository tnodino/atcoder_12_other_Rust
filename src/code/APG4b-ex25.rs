// https://atcoder.jp/contests/APG4b/tasks/APG4b_bx

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn intersection(A: &[bool; 50], B: &[bool; 50], flg: &mut [bool; 50]) {
    for i in 0..50 {
        flg[i] = A[i] & B[i];
    }
}

#[allow(non_snake_case)]
fn union_set(A: &[bool; 50], B: &[bool; 50], flg: &mut [bool; 50]) {
    for i in 0..50 {
        flg[i] = A[i] | B[i];
    }
}

#[allow(non_snake_case)]
fn symmetric_diff(A: &[bool; 50], B: &[bool; 50], flg: &mut [bool; 50]) {
    for i in 0..50 {
        flg[i] = A[i] ^ B[i];
    }
}

#[allow(non_snake_case)]
fn subtract(A: &[bool; 50], x: usize, flg: &mut [bool; 50]) {
    for i in 0..50 {
        if i == x {
            continue;
        }
        flg[i] |= A[i];
    }
}

#[allow(non_snake_case)]
fn increment(A: &[bool; 50], flg: &mut [bool; 50]) {
    for i in 0..50 {
        flg[(i+1)%50] |= A[i];
    }
}

#[allow(non_snake_case)]
fn decrement(A: &[bool; 50], flg: &mut [bool; 50]) {
    for i in 0..50 {
        flg[(i+49)%50] |= A[i];
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = [false; 50];
    for _ in 0..N {
        input! {
            a: usize,
        }
        A[a] = true;
    }
    input! {
        M: usize,
    }
    let mut B = [false; 50];
    for _ in 0..M {
        input! {
            b: usize,
        }
        B[b] = true;
    }
    input! {
        command: String,
    }
    let mut flg = [false; 50];
    match command.as_str() {
        "intersection" => intersection(&A, &B, &mut flg),
        "union_set" => union_set(&A, &B, &mut flg),
        "symmetric_diff" => symmetric_diff(&A, &B, &mut flg),
        "subtract" => {
            input! {
                x: usize,
            }
            subtract(&A, x, &mut flg);
        },
        "increment" => increment(&A, &mut flg),
        _ => decrement(&A, &mut flg),
    }
    let mut ans = Vec::new();
    for i in 0..50 {
        if flg[i] {
            ans.push(i);
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}