use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for (num, freq) in freq_map {
            heap.push(Reverse((freq, num)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut result = Vec::new();
        while let Some(Reverse((_, num))) = heap.pop() {
            result.push(num);
        }
        result.reverse();
        result
    }
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}
