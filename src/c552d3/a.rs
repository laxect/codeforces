/**
 * https://codeforces.com/contest/1154/problem/A
 */
pub fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("error!");
    let mut nums = Vec::new();
    let mut max = 0;
    for item in buffer.split_whitespace() {
        let num: i32 = item.parse().expect("invalid number");
        nums.push(num);
        max = std::cmp::max(num, max);
    }
    let mut out = String::new();
    for item in nums.into_iter() {
        if item != max {
            out.push_str(&format!("{} ", max - item));
        }
    }
    println!("{}", out.trim());
}