use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }
    let a_b = a*a + b*b;
    let c_c = c*c;
    let ans = if a_b < c_c {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}