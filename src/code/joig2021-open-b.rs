// https://atcoder.jp/contests/joig2021-open/tasks/joig2021_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        T: String,
    }
    let T = T.chars().collect::<Vec<char>>();
    for i in 0..K-1 {
        print!("{}", T[i]);
    }
    for i in K-1..N {
        match T[i].is_uppercase() {
            true => print!("{}", T[i].to_lowercase()),
            false => print!("{}", T[i].to_uppercase()),
        }
    }
    println!();
}