// https://atcoder.jp/contests/practice2/tasks/practice2_d

use proconio::input;
use proconio::fastout;
use ac_library::MfGraph;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let s = N * M;
    let t = N * M + 1;
    let mut MG = MfGraph::new(N * M + 2);
    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 0 {
                MG.add_edge(s, i * M + j, 1);
            }
            else {
                MG.add_edge(i * M + j, t, 1);
            }
        }
    }
    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 1 {
                continue;
            }
            if S[i][j] == '#' {
                continue;
            }
            if i + 1 < N && S[i+1][j] == '.' {
                MG.add_edge(i * M + j, (i + 1) * M + j, 1);
            }
            if 0 < i && S[i-1][j] == '.' {
                MG.add_edge(i * M + j, (i - 1) * M + j, 1);
            }
            if j + 1 < M && S[i][j+1] == '.' {
                MG.add_edge(i * M + j, i * M + (j + 1), 1);
            }
            if 0 < j && S[i][j-1] == '.' {
                MG.add_edge(i * M + j, i * M + (j - 1), 1);
            }
        }
    }
    println!("{}", MG.flow(s, t));
    for e in MG.edges() {
        if e.from == s || e.to == t || e.flow == 0 {
            continue;
        }
        let from_x = e.from / M;
        let from_y = e.from % M;
        let to_x = e.to / M;
        let to_y = e.to % M;
        if to_x + 1 < N && from_x == to_x + 1 {
            S[from_x][from_y] = '^';
            S[to_x][to_y] = 'v';
        }
        if 0 < to_x && from_x == to_x - 1 {
            S[from_x][from_y] = 'v';
            S[to_x][to_y] = '^';
        }
        if to_y + 1 < M && from_y == to_y + 1 {
            S[from_x][from_y] = '<';
            S[to_x][to_y] = '>';
        }
        if 0 < to_y && from_y == to_y - 1 {
            S[from_x][from_y] = '>';
            S[to_x][to_y] = '<';
        }
    }
    for i in 0..N {
        println!("{}", S[i].iter().collect::<String>());
    }
}