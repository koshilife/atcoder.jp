// https://atcoder.jp/contests/dp/tasks/dp_a

const INF: i64 = 1001001001001001;

fn main(){
  proconio::input!{ n:usize, h:[isize; n] }
  let mut dp = vec![0; n];
  dp[1] = (h[0] - h[1]).abs();
  for i in 1..n {
      dp[i] = std::cmp::min(
        dp[i-1] + (h[i] - h[i-1]).abs(),
        dp[i-2] + (h[i] - h[i-2]).abs()
      );
  }
  println!("{}", dp[n-1]);
}
