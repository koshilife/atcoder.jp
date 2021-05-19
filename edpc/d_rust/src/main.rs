// https://atcoder.jp/contests/dp/tasks/dp_d

// 公式解説より
// dp[i][j]= (i番目までの品物を、重さがj以下になるように選んだときの価値の最大値)

fn main(){
    proconio::input!{
        n: usize,
        w: usize,
        scores: [[usize; 2]; n],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; n+1];
    // 対象品数を増やしていく
    for i in 1..=n {
      let w_i = scores[i-1][0];
      let v_i = scores[i-1][1];
      // 扱える重さを増やしていく
      for j in 1..=w {
        if j >= w_i {
          // キャパ以内の重さなら、加えたときの価値総量と比較
          dp[i][j] = dp[i-1][j].max(dp[i-1][j-w_i] + v_i);
        } else {
          // キャパ未満ならスルー
          dp[i][j] = dp[i-1][j];
        }
      }
    }
    println!("{}", dp[n][w]);
}
