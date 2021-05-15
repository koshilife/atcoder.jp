use proconio::input;

fn main() {
    input! {
        n: i32,
        mut mountains: [(String, i32); n],
    }
    mountains.sort_by(|a, b| a.1.cmp(&b.1));
    let ans = &mountains[mountains.len() - 2].0;
    println!("{}", ans);
}