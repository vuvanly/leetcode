struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let length = nums.len();
        if length < 3 {
            return length as i32;
        }
        let mut k: usize = 2;

        for index in 2..length {
            if nums[index] != nums[k-2] {
                nums[k] = nums[index];
                // if k != index {
                //     nums[index] = -10001;
                // }
                k += 1;
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1,1,1,2,2,3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        // assert_eq!(nums, vec![1,1,2,2,3,20000]); 1 1 2  2

        let mut nums = vec![0,0,1,1,1,1,2,3,3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 7);
        // assert_eq!(nums, vec![0,0,1,1,2,3,3,20000,20000]);
    }
}