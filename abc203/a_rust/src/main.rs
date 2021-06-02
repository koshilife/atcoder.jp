fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32
    }
    let ans = if a == b {
        c
    } else if a == c {
        b
    } else if b == c {
        a
    } else {
        0
    };
    println!("{}", ans);
}