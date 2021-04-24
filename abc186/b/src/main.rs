use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }
    let mut sum = 0;
    let mut min = std::i32::MAX;
    for row in &a {
        for value in row {
            sum += value;
            min = min.min(*value);
        }
    }
    let total = h as i32 * w as i32;
    println!("{}", sum - min * total);
}