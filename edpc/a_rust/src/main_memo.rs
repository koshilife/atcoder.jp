// https://atcoder.jp/contests/dp/tasks/dp_a

// 勉強メモ:
// 足場 H(n)に到着する方法は H(n-1), H(n-2)どちらかしかない。
// n-1, n-2 どちらからの移動の最短コストの総和を最後の足場から計算して求める。

// メモ化再起での解法

const INF: i64 = 1001001001001001;

fn get_cost(i: usize, hn: &Vec<i64>, memo: &mut Vec<i64>) -> i64 {
  if i == 0 { return 0; }
  if memo[i] != INF { return memo[i]; }

  let mut ans = INF;
  if i >= 1 {
    let jump_cost = (hn[i] - hn[i-1]).abs();
    ans = ans.min(jump_cost + get_cost(i-1, hn, memo));
  }
  if i >= 2 {
    let jump_cost = (hn[i] - hn[i-2]).abs();
    ans = ans.min(jump_cost + get_cost(i-2, hn, memo));
  }
  memo[i] = ans;
  return ans;
}

fn main() {
    proconio::input! {
        n: usize,
        hn: [i64; n],
    }
    let mut memo: Vec<i64> = vec![INF; n];
    println!("{}", get_cost(n - 1, &hn, &mut memo));
}
