fn total(n:u64) -> u64 {
    return (n+1)*n/2
}

fn main() {
    proconio::input! {
        n: u64
    }
    let mut ans = 1;
    loop {
        if total(ans) >= n { break; }
        ans += 1;
    }
    println!("{}", ans)
}
