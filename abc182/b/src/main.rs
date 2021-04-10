use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut kmax = -1;
    let mut ans = -1;
    for k in 2..=1000 {
        let mut cnt = 0;
        for i in &a {
            if i % k == 0 {
                cnt += 1;
            }
        }
        if cnt > kmax {
            kmax = cnt;
            ans = k;
        }
    }
    println!("{}", ans);
}