use std::{collections::HashMap, vec};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for num in 0..nums.len() {
            for inner in 0..nums.len() {
                let sum = nums[num] + nums[inner];
                if sum == target && num != inner {
                    return vec![num as i32, inner as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // x = target - y
        // Map to store the key pairs of the seen values, to be used to lookup the
        // corresponding key value if a matching valur
        let mut seen = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&index) = seen.get(&diff) {
                // If found, return the index of the complement and the current index
                return vec![index as i32, i as i32];
            }
            // If not found, add the current value to the map
            seen.insert(num, i);
        }
        return vec![];
    }
}

fn main() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
