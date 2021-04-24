use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
        bn: [i32; n],
    }
    // println!("{}", n);
    // println!("{:?}", an);
    // println!("{:?}", bn);
    let mut min = 1;
    let mut max = 1000;
    for i in 0..n {
        min = min.max(an[i]);
        max = max.min(bn[i]);
    }
    // println!("min:{}", min);
    // println!("max:{}", max);
    let mut cnt = 0;
    for _i in min..=max { cnt += 1}
    println!("{}", cnt);
}