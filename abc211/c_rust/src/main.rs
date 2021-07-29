//
// DP解法
// 与えられる文字列S、目標とする文字列T。
// dp[i][j]: 文字列Sのi文字目までで作る、文字列Tの1文字からjまでの部分文字列を選ぶ組み合わせ数とする。
//
fn main() {
    proconio::input! {
        s: String
    }
    let s_len = s.len();
    let s_chars = s.chars().collect::<Vec<char>>();

    let target_str = "chokudai";
    let target_len = target_str.len();
    let target_chars = target_str.chars().collect::<Vec<char>>();

    let mod_num = 10u64.pow(9) + 7;
    let mut dp = vec![vec![0 as u64; target_len]; s_len];

    for j in 0..target_len {
        for i in 0..s_len {
            if s_chars[i] != target_chars[j] {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j];
                }
            } else {
                if i >= j {
                    if j == 0 {
                        if i == 0 {
                            dp[i][j] += 1;
                        } else {
                            dp[i][j] = dp[i - 1][j] + 1;
                        }
                    } else {
                        dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % mod_num;
                    }
                }
            }
        }
    }
    // debug
    // println!("DEBUG DP");
    // for j in 0..target_len {
    //     let mut a = Vec::new();
    //     for i in 0..s_len {
    //         a.push(dp[i][j]);
    //     }
    //     println!("j:{} => {:?}", j, a);
    // }
    println!("{}", dp[s_len - 1][target_len - 1])
}
