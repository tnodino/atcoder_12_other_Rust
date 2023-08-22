// https://atcoder.jp/contests/past202303-open/tasks/past202303_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        A: String,
        B: String,
    }
    let Al = &A[..A.len()-D-1].parse::<usize>().unwrap();
    let Ar = &A[A.len()-D..].chars().collect::<Vec<char>>();
    let Bl = &B[..B.len()-D-1].parse::<usize>().unwrap();
    let Br = &B[B.len()-D..].chars().collect::<Vec<char>>();
    let mut Cl = Al + Bl;
    let mut Cr = vec![0; D];
    for i in (0..D).rev() {
        Cr[i] += ((Ar[i] as usize) - ('0' as usize)) + ((Br[i] as usize) - ('0' as usize));
        if i == 0 {
            Cl += Cr[i] / 10;
        }
        else {
            Cr[i-1] += Cr[i] / 10;
        }
        Cr[i] %= 10;
    }
    println!("{}.{}", Cl, Cr.iter().map(|&x| x.to_string()).collect::<String>());
}