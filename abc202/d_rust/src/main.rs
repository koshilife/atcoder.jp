// https://atcoder.jp/contests/abc200/tasks/abc202_d
// 公式解説を元に作成

fn main() {
    proconio::input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    }
    // コンビネーションテーブルを作成
    let mut c: Vec<Vec<usize>> = vec![vec![0; 60+1]; 60+1];
    c[0][0] = 1;
    for i in 0..60 {
        for j in 0..=i {
            c[i + 1][j] += c[i][j];
            c[i + 1][j + 1] += c[i][j];
        }
    }
    // 先頭文字から決めていく
    let mut ans = String::new();
    while a + b > 0 {
        let x = if a >= 1 { c[a + b - 1][a - 1] } else { 0 };
        if k <= x {
            ans += "a";
            a -= 1;
        } else {
           ans += "b";
           b -= 1;
           k -= x;
        }
    }
    println!("{}", ans);
}