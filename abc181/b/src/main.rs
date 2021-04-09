use proconio::input;

// fn main() {
//   input! {
//       n: usize,
//       v: [(i64, i64); n],
//   }
//   let mut ans = 0;
//   for (a, b) in &v {
//       ans += (a + b) * (b - a + 1) / 2;
//   }
//   println!("{}", ans);
// }

// fn main() {
//     input! {
//         v: [(i64, i64)],
//     }
//     let mut sum = 0;
//     for (a, b) in &v {
//         sum += (a + b) * (b - a + 1) / 2;
//     }
//     println!("{}", sum);
// }

fn main() {
    input! {
        v: [(i64, i64)],
    }
    let ans: i64 = v.iter().map(|(x, y)| (x + y) * (y - x + 1) / 2).sum();
    println!("{}", ans);
}