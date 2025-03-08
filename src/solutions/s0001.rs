use std::collections::HashMap;


pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();
        for i in 0..length {
            for j in i+1..length {
                if nums[i] + nums[j] == target {
                    return Vec::from([i as i32, j as i32]);
                }
            }
        }
        return Vec::from([-1, -1]);
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let length = nums.len();
        for i in 0..length {
            let val = nums[i];
            let diff = target-val;
            if map.contains_key(&(target-val)) {
                return Vec::from([map.get(&diff).unwrap().clone(), i as i32]);
            }
            map.insert(val, i as i32);
        }
        return Vec::from([-1, -1]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::two_sum(Vec::from([2, 7, 4, 3]), 9);
        println!("{:?}", res);
    }
}
