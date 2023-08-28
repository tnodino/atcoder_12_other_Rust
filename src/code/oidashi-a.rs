// https://atcoder.jp/contests/oidashi/tasks/oidashi_a

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[ 0, 0,!0, 1];
const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'S' {
                sx = i;
                sy = j;
                S[i][j] = '.';
            }
            else if S[i][j] == 'G' {
                gx = i;
                gy = j;
                S[i][j] = '.';
            }
        }
    }
    let mut scost = vec![vec![INF; W]; H];
    scost[sx][sy] = 0;
    let mut que = VecDeque::new();
    que.push_back((sx, sy));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if S[nx][ny] == '#' {
                continue;
            }
            if scost[x][y] + 1 < scost[nx][ny] {
                scost[nx][ny] = scost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    let mut gcost = vec![vec![INF; W]; H];
    gcost[gx][gy] = 0;
    let mut que = VecDeque::new();
    que.push_back((gx, gy));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if S[nx][ny] == '#' {
                continue;
            }
            if gcost[x][y] + 1 < gcost[nx][ny] {
                gcost[nx][ny] = gcost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '.' {
                continue;
            }
            let mut res1 = INF;
            let mut res2 = INF;
            for d in 0..4 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if H <= nx || W <= ny {
                    continue;
                }
                res1 = min(res1, scost[nx][ny]);
                res2 = min(res2, gcost[nx][ny]);
            }
            if res1 + res2 >= INF {
                continue;
            }
            ans = max(ans, res1 + res2 + 2);
        }
    }
    println!("{}", ans);
}