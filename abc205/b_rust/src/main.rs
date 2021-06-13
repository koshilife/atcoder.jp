fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = "Yes";
    for (i, val) in a.iter().enumerate() {
        if i+1 != *val {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}