// https://atcoder.jp/contests/APG4b/tasks/APG4b_by

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut hour: isize,
        mut minute: isize,
        mut second: isize,
        mut diff_second: isize,
    }
    println!("{:02}:{:02}:{:02}", hour, minute, second);
    while 0 < diff_second {
        second += 1;
        if second == 60 {
            second = 0;
            minute += 1;
            if minute == 60 {
                minute = 0;
                hour += 1;
                if hour == 24 {
                    hour = 0;
                }
            }
        }
        diff_second -= 1;
    }
    while diff_second < 0 {
        second -= 1;
        if second == -1 {
            second = 59;
            minute -= 1;
            if minute == -1 {
                minute = 59;
                hour -= 1;
                if hour == -1 {
                    hour = 23;
                }
            }
        }
        diff_second += 1;
    }
    println!("{:02}:{:02}:{:02}", hour, minute, second);
}