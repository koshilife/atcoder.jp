use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans_chars:Vec<char> = Vec::new();
    for c in s.chars().rev() {
        if c == '6' {
            ans_chars.push('9');
        } else if c == '9' {
            ans_chars.push('6');
        } else {
            ans_chars.push(c);
        }
    }
    let ans: String = ans_chars.iter().collect();
    println!("{}", ans);
}