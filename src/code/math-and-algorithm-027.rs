// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aa

use proconio::input;
use proconio::fastout;

// A[l], A[l+1], ..., A[r-1] を小さい順に整列する関数
#[allow(non_snake_case)]
fn MergeSort(l: usize, r: usize, A: &mut Vec<usize>, C: &mut Vec<usize>) {
    // r - l = 1 の場合、すでにソートされているので何もしない
    if r - l == 1 {
        return;
    }
    // 2 つに分割した後、小さい配列をソート
    let m = (l + r) / 2;
    MergeSort(l, m, A, C);
    MergeSort(m, r, A, C);
    // この時点で以下の 2 つの配列がソートされている：
    // 列 A' に相当するもの [A[l], A[l+1], ..., A[m-1]]
    // 列 B' に相当するもの [A[m], A[m+1], ..., A[r-1]]
    // 以下が Merge 操作となります。
    let mut c1 = l;
    let mut c2 = m;
    let mut cnt = 0;
    while c1 != m || c2 != r {
        if c1 == m {
            // 列 A' が空の場合
            C[cnt] = A[c2];
            c2 += 1;
        }
        else if c2 == r {
            // 列 B' が空の場合（抜けている部分）
            C[cnt] = A[c1];
            c1 += 1;
        }
        else {
            // そのいずれでもない場合（抜けている部分）
            if A[c1] < A[c2] {
                C[cnt] = A[c1];
                c1 += 1;
            }
            else {
                C[cnt] = A[c2];
                c2 += 1;
            }
        }
        cnt += 1;
    }
    // 列 A', 列 B' を合併した配列 C を A に移す
    // [C[0], ..., C[cnt-1]] -> [A[l], ..., A[r-1]]
    for i in 0..cnt {
        A[l+i] = C[i];
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    // 入力
    input! {
        N: usize,
        mut A: [usize; N],
    }
    let mut C = vec![0; N];
    // マージソート -> 答えの出力
    MergeSort(0, N, &mut A, &mut C);
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}