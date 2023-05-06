// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    'outer: for i in 2..=N {
        for j in 2..i {
            if i % j == 0 {
                continue 'outer;
            }
        }
        vec.push(i);
    }
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}