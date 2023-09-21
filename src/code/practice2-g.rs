// https://atcoder.jp/contests/practice2/tasks/practice2_g

use proconio::input;
use proconio::fastout;
use ac_library::SccGraph;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut G = SccGraph::new(N);
    for _ in 0..M {
        input! {
            (a, b): (usize, usize),
        }
        G.add_edge(a, b)
    }
    let scc = G.scc();
    println!("{}", scc.len());
    for i in 0..scc.len() {
        println!("{} {}", scc[i].len(), scc[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}