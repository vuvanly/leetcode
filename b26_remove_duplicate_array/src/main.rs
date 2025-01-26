struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // [0,0,1,1,1,2,2,3,3,4]
        let mut k = 1;
        let length = nums.len();
        let mut last_number = nums[0];
        for index in 1..length {
            if nums[index] != last_number {
                nums[k] = nums[index];
                k+=1;
                last_number = nums[index];
            }
        }
        for index in k..length {
            nums[index] = 999;
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1,1,2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(nums, vec![1,2,999]);

        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums, vec![0,1,2,3,4,999,999,999,999,999]);
    }
}
