// https://atcoder.jp/contests/dp/tasks/dp_c

//
// <DPの定義>
// 公式解説より
// dp[i][j]=(i日目に行動jをして、i日目までに得られる幸福度の和の最大値)
//
// <勉強メモ>
// - 前日行動3パターン毎にスコア合計を最初から最後まで順に積み上げていく形で解ける。
// - 計算量は O(n)
// - scores をタプル [(usize, usize, usize); n] 的 で受けると、配列の添字にアクセスしにくくなるので
//   2重配列で受けた点は工夫した。
//

fn main(){
  proconio::input!{
      n:usize,
      scores: [[usize; 3]; n],
  }
  let mut dp = vec![vec![0; 3]; n];
  for i in 0..n {
    for j in 0..3 {
      let before_score = if i == 0 { 0 } else { dp[i-1][j] };
      for k in 0..3 {
        if j == k { continue; }
        dp[i][k] = dp[i][k].max(before_score + scores[i][k]);
      }
    }
  }
  let mut ans = 0;
  for j in 0..3 {
    ans = dp[n-1][j].max(ans);
  }
  println!("{}", ans);
}
