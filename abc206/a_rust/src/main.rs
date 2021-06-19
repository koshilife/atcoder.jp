fn main() {
    proconio::input! {
        a: f64,
    }
    let b = (a * 1.08).floor() as i32;
    let ans = if b == 206 {
        "so-so"
    } else if b > 206 {
        ":("
    } else {
        "Yay!"
    };
    println!("{}", ans);
}