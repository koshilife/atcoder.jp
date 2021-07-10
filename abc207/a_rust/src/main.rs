fn main() {
    proconio::input! {
        mut nums: [i32; 3],
    }
    nums.sort();
    println!("{}", nums[1] + nums[2]);
}