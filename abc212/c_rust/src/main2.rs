// 公式解説の解法

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut an: [i64; n],
        mut bn: [i64; m],
    }
    an.sort();
    bn.sort();

    let mut cursor_a = 0;
    let mut cursor_b = 0;
    let mut ans = 10i64.pow(13);
    while cursor_a < n && cursor_b < m {
        let a = an[cursor_a];
        let b = bn[cursor_b];
        if a == b {
            ans = 0;
            break;
        }
        ans = ans.min((a - b).abs());
        if a < b {
            cursor_a += 1;
        } else {
            cursor_b += 1;
        }
    }
    println!("{}", ans);
}
