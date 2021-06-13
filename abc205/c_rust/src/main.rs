fn compare(a: isize, b: isize) -> isize {
    if a == b {
        0
    } else if a > b {
        1
    } else {
        -1
    }
}

fn main() {
    proconio::input! {
        a: isize,
        b: isize,
        c: isize,
    }
    let ans:isize;
    if c % 2 == 0 {
        ans = compare(a.abs(), b.abs());
    } else {
        let a_is_nega = a < 0;
        let b_is_nega = b < 0;
        if a_is_nega && b_is_nega {
            ans = -compare(a.abs(), b.abs());
        } else if a_is_nega && !b_is_nega {
            ans = -1;
        } else if !a_is_nega && b_is_nega {
            ans = 1;
        } else {
            // a >= 0, b >= 0
            ans = compare(a.abs(), b.abs());
        }
    }
    println!("{}", if ans == 0 { '=' } else if ans == 1 { '>' } else { '<' });
}