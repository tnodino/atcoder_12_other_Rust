// https://atcoder.jp/contests/atc001/tasks/dfs_a

use proconio::input;
use proconio::fastout;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[0, 0, !0, 1];

#[allow(non_snake_case)]
fn dfs(x: usize, y: usize, H: usize, W: usize, flg: &mut Vec<Vec<bool>>, c: &Vec<Vec<char>>) {
    for d in 0..4 {
        let nx = x.wrapping_add(DX[d]);
        let ny = y.wrapping_add(DY[d]);
        if H <= nx || W <= ny {
            continue;
        }
        if flg[nx][ny] {
            continue;
        }
        if c[nx][ny] == '#' {
            continue;
        }
        flg[nx][ny] = true;
        dfs(nx, ny, H, W, flg, c);
    }
    return;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut c = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..H {
        for j in 0..W {
            if c[i][j] == 's' {
                sx = i;
                sy = j;
                c[i][j] = '.';
            }
            if c[i][j] == 'g' {
                gx = i;
                gy = j;
                c[i][j] = '.';
            }
        }
    }
    let mut flg = vec![vec![false; W]; H];
    flg[sx][sy] = true;
    dfs(sx, sy, H, W, &mut flg, &c);
    if flg[gx][gy] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}