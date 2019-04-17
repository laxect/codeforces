/**
 * https://codeforces.com/contest/1154/problem/B
 */
macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        std::io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

pub fn main() {
    let _ = parse_line!(i32);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("n not esist");
    let mut nums: Vec<i32> = buffer.split_whitespace().map(|x| x.parse().expect("error")).collect();
    nums.sort();
    nums.dedup();
    let res = match nums.len() {
        1 => 0,
        2 => {
            let d = nums[1] - nums[0];
            if d % 2 == 0 {
                d / 2
            } else {
                d
            }
        }
        3 => {
            if nums[2] - nums[1] == nums[1] - nums[0] {
                nums[2] - nums[1]
            } else {
                -1
            }
        }
        _ => -1,
    };
    println!("{}", res);
}