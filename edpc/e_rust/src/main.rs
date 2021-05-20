// https://atcoder.jp/contests/dp/tasks/dp_e

// 公式解説より
// dp[i][j]=(i番目までの品物を、価値がjになるように選んだときの最小重さ)
// => Wの制約が 10**9 のため、Dの手法では計算できない。
//   => 制約に応じて 問題Dとのdp漸化式のw,vを入れ替えて計算量を調整する
//
// ハマった点:
// - MAX_VALUEの桁数間違えで1時間溶ける

const MAX_VALUE: usize = 100000;

fn main(){
    proconio::input!{
        n: usize,
        w: usize,
        wv: [[usize; 2]; n],
    }
    let mut dp = vec![vec![10_isize.pow(11) as usize; MAX_VALUE+1]; n+1];
    dp[0][0] = 0;
    for i in 1..=n {
      let _w = wv[i-1][0];
      let _v = wv[i-1][1];
      for j in 0..=MAX_VALUE {
        dp[i][j] = dp[i-1][j];
        if j >= _v {
          dp[i][j] = dp[i][j].min(dp[i-1][j-_v] + _w) ;
        }
      }
    }
    let mut ans = MAX_VALUE;
    while dp[n][ans] > w {
        ans -= 1;
        if ans == 0 { break; }
    }
    println!("{}", ans);
}
