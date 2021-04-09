use proconio::input;

fn main() {
    input! {
        a: [i32; 4],
    }
    println!("{}", a.iter().min().unwrap());
}
// fn main() {
//     input! {
//         a: [i32; 4],
//     }
//     let mut ans = 100;
//     for &i in &a {
//         ans = ans.min(i);
//     }
//     println!("{}", ans);
// }