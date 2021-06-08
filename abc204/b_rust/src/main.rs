fn main() {
    proconio::input! {
        n: i32,
        a: [i32; n],
    }
    let mut total = 0;
    for i in a {
        if i <= 10 { continue; }
        total += i - 10;
    }
    println!("{}", total);
}