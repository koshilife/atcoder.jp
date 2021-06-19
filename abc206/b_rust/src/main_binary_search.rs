fn total(n:u64) -> u64 {
    return (n+1)*n/2
}

fn main() {
    proconio::input! {
        n: u64
    }
    if n == 1 {
        println!("{}", 1);
        return;
    }
    let mut upper = 100000;
    let mut lower = 1;
    while lower + 1 < upper {
        let x = (lower+upper)/2;
        let t = total(x);
        if n == t {
            println!("{}", x);
            return;
        } else if n > t {
            lower = x;
        } else {
            upper = x;
        }
    }
    println!("{}", lower + 1);
}
