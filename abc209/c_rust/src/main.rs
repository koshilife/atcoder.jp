fn main() {
    proconio::input! {
        n: usize,
        mut cn: [u64; n]
    }
    let amari:u64 = 10u64.pow(9) + 7;
    cn.sort();
    if cn[n-1] < n as u64 {
        println!("{}", 0);
    } else {
        let mut ans:u64 = cn[0];
        for i in 1..=n-1 {
            ans = (ans * (cn[i] - i as u64)) % amari;
        }
        println!("{}", ans);
    }
}
