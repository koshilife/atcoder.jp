use proconio::input;

fn do_query(n: usize, s: &String, query: (usize, usize, usize)) -> String {
    let mut _s = String::new();
    if query.0 == 1 {
        let a = query.1 - 1;
        let b = query.2 - 1;
        let s1 = &s[..a];
        let s2 = &s[a..(a+1)];
        let s3 = &s[(a+1)..b];
        let s4 = &s[b..(b+1)];
        let s5 = &s[(b+1)..];
        _s += s1;
        _s += s4;
        _s += s3;
        _s += s2;
        _s += s5;
        // println!("a: {}", a);
        // println!("b: {}", b);
        // println!("s1: {}", s1);
        // println!("s2: {}", s2);
        // println!("s3: {}", s3);
        // println!("s4: {}", s4);
        // println!("s5: {}", s5);
        return _s
    } else {
        let mut _s = String::new();
        let s1 = &s[..n];
        let s2 = &s[n..];
        _s += s2;
        _s += s1;
        return _s
    }
}

fn main() {
    input! {
        n: usize,
        s: String,
        q: usize,
        queries: [(usize, usize, usize); q]
    }
    // println!("{}", n);
    // println!("{}", s);
    // println!("{}", q);
    // println!("{:?}", queries);
    let mut str = s;
    for query in queries {
        str = do_query(n, &str, query);
        // println!("{}", str);
    }
    println!("{}", str);
}