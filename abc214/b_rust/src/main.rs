fn main() {
    proconio::input! {
        s: usize,
        t: usize
    }
    let mut ans = 0;
    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1
                }
            }
        }
    }
    println!("{}", ans);
}
