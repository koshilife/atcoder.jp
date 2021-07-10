fn main() {
    proconio::input! {
        a: i32,
        b: i32
    }
    let mut ans = 0;
    if a <= b {
        ans = b - a + 1;
    }
    println!("{}", ans);
}