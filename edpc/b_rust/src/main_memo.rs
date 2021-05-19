// https://atcoder.jp/contests/dp/tasks/dp_b

// 勉強メモ:
// Frog1 の ジャンプ先が2個の固定ではなく動的なパターン

メモ化再起での解法

const INF: isize = 1001001001001001;

fn get_cost(i: usize, k: usize, hn: &Vec<i64>, memo: &mut Vec<i64>) -> i64 {
  if i == 0 { return 0; }
  if memo[i] != INF { return memo[i]; }

  let mut ans = INF;
  for j in 1..=k {
      if i < j { break; }
      let jump_cost = (hn[i] - hn[i-j]).abs();
      ans = ans.min(jump_cost + get_cost(i-j, k, hn, memo));
  }
  memo[i] = ans;
  return ans;
}

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        hn: [i64; n],
    }
    let mut memo: Vec<i64> = vec![INF; n];
    println!("{}", get_cost(n - 1, k, &hn, &mut memo));
}
