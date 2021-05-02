use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let v:Vec<&str> = s.split("ZONe").collect();
    println!("{}", v.len() - 1);
}
