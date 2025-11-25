struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_mp = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_mp.get(&complement) {
                return vec![index as i32, i as i32];
            }
            num_mp.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
