fn main() {
    proconio::input! {
        n: i64,
        k: i64,
    }
    let mut ans: i64 = 0;
    for i in 1..=n {
        for j in 1..=k {
            ans += i * 100 + j
        }
    }
    println!("{}", ans);
}