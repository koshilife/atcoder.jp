use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    proconio::input! { q: usize }
    let mut heap = BinaryHeap::new();
    let mut add: i64 = 0;
    for _ in 0..q {
        proconio::input! { p: usize }
        match p {
            1 => {
                proconio::input! { x: i64 }
                heap.push(Reverse(x - add));
            }
            2 => {
                proconio::input! { x: i64 }
                add += x;
            }
            3 => {
                if let Some(Reverse(v)) = heap.pop() {
                    println!("{}", v + add);
                }
            }
            _ => unreachable!(),
        }
    }
}
