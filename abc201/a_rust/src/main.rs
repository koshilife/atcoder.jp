use proconio::input;

fn main() {
    input! {
        mut n: [i32; 3],
    }
    n.sort();
    let d1_2 = n[0] - n[1];
    let d2_3 = n[1] - n[2];
    let ans = if d1_2 == d2_3 { "Yes" } else { "No" };
    println!("{}", ans);
}