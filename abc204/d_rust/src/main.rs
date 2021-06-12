// v1の改良版: 以下を参考に改良
// https://note.com/momomo0214/n/n18a3c1872aff
// 2次元配列から1次元にして、メモリ・実行時間を効率化。


fn main() {
    proconio::input! {
        n: usize,
        t: [usize; n]
    }
    let total = t.iter().sum::<usize>();
    // dp[j]: 片方のオーブンAにおいて、最後の料理までの調理時間が j分にすることができるか。
    let mut dp = vec![false; total+1];
    dp[0] = true;
    for i in 1..=n {
        // 調理時間が大きい方からDP更新を行うように変更
        for j in (0..=total).rev() {
            if dp[j] {
                dp[j + t[i-1]] = true;
            }
        }
    }
    let mut ans = total;
    for j in (total/2)..=total {
        if dp[j] {
            ans = ans.min(j.max(total-j));
        }
    }
    println!("{}", ans);
}

