// https://atcoder.jp/contests/abc204/tasks/abc204_d
// 公式解説を元に実装
//
// <制約>
// N <= 10**2
// ti <= 10**3
// Total <= 10**5
//
// 解法:部分和問題に帰着させる
// - 計算量: O(N x T) => 10**7
//
// Σb = Total - Σa
// で一位に決まる。

fn main() {
    proconio::input! {
        n: usize,
        t: [usize; n]
    }
    let total = t.iter().sum::<usize>();
    // dp[i][j]: 片方のオーブンAにおいて、料理i個目までの調理時間が j分にすることができるか。
    let mut dp = vec![vec![false; total+1]; n+1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=total {
            if dp[i-1][j] {
                // 一つ前の料理iがJ分だったものがあればDP更新
                // 更新1: オーブンAで調理しない
                dp[i][j] = true;
                // 更新2: オーブンAで調理する (調理時間追加)
                dp[i][j + t[i-1]] = true;
            }
        }
    }
    let mut ans = total;
    // 理論上のアンサーの最小値となりうる total/2 から探索
    for j in (total/2)..=total {
        if dp[n][j] {
            // オーブンAとオーブンBの最大の調理時間が小さい方で最小値を比較してAnswerを更新
            ans = ans.min(j.max(total-j));
        }
    }
    println!("{}", ans);
}

