impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        let mut j = 0;
        while i < nums.len() {
            while (i < nums.len()) && (nums[i] == nums[j]) {
                i += 1;
            }
            if i < nums.len() {
                j += 1;
                nums[j] = nums[i];
                i += 1;
            }
        }
        (j + 1) as i32
    }
}