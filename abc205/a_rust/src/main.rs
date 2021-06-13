fn main() {
    proconio::input! {
        a: f64,
        b: f64,
    }
    let ans = b * (a/100.0);
    println!("{}", ans);
}