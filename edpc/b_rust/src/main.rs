// https://atcoder.jp/contests/dp/tasks/dp_b

const INF: isize = 1001001001001001;

fn main(){
  proconio::input!{ n:usize, k:usize, h:[isize; n] }
  let mut dp = vec![INF; n];
  dp[0] = 0;

  for i in 1..n {
    for j in 1..=k {
        if j > i { break; }
        dp[i] = dp[i].min(dp[i-j] + (h[i] - h[i-j]).abs());
    }
  }
  println!("{}", dp[n-1]);
}
