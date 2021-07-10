
fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64
    }
    let mut blue = a;
    let mut red = 0;
    let mut ans = 0;

    if b >= c * d {
        println!("{}", -1);
        return;
    }

    loop {
        if blue <= red * d { break; }
        ans += 1;
        blue += b;
        red += c;
    }
    println!("{}", ans);
}
// 5 2 3 2
//     blue   red  red *d  blue >= red*d
// 0:  5       0   0       false
// 1:  7       3   6       false
// 2:  9       6   12      true
