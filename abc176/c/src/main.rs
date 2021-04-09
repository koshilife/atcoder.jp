
// use proconio::input;
// fn main() {
//     input! {
//         a: [i64],
//     }
//     let mut max = 0;
//     let mut ans = 0;
//     for i in &a {
//         max = max.max(*i);
//         ans += max - i;
//     }
//     println!("{}", ans);
// }

// iterator pattern
use proconio::input;

fn main() {
    input! {
        a: [i64],
    }
    let ans: i64 = a
        .iter()
        .scan(0, |max, &x| {
            *max = x.max(*max);
            Some(*max - x)
        })
        .sum();
    println!("{}", ans);
}