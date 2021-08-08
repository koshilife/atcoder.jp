fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let mut ans = -1;
    for i in 0..256 {
        if a ^ i == b {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
