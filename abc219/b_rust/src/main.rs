fn main() {
    proconio::input! {
        s1: String,
        s2: String,
        s3: String,
        t: String
    }
    let mut ans = "".to_string();
    for ti in t.chars() {
        match ti {
            '1' => ans = format!("{}{}", ans, s1),
            '2' => ans = format!("{}{}", ans, s2),
            '3' => ans = format!("{}{}", ans, s3),
            _ => {}
        }
    }
    println!("{}", ans);
}
