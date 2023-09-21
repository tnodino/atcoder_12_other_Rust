// https://atcoder.jp/contests/practice2/tasks/practice2_e

use proconio::input;
use proconio::fastout;
use ac_library::MinCostFlowGraph;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
    }
    let M = N as isize;
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            a: [isize; N],
        }
        A.push(a);
    }
    let s = N * 2;
    let t = N * 2 + 1;
    let mut MG = MinCostFlowGraph::new(N * 2 + 2);
    MG.add_edge(s, t, M * K, INF);
    for i in 0..N {
        MG.add_edge(s, i, K, 0);
        MG.add_edge(N + i, t, K, 0);
        for j in 0..N {
            MG.add_edge(i, N + j, 1, INF - A[i][j]);
        }
    }
    let res = MG.flow(s, t, M * K);
    println!("{}", M * K * INF - res.1);
    let mut grid = vec![vec!['.'; N]; N];
    for e in MG.edges() {
        if e.from == s || e.to == t || e.flow == 0 {
            continue;
        }
        let x = e.from;
        let y = e.to - N;
        grid[x][y] = 'X';
    }
    for i in 0..N {
        println!("{}", grid[i].iter().collect::<String>());
    }
}