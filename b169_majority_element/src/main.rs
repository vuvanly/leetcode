struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // if let mut map = HashMap::new();
        // for num in nums.iter() {
        //     let count = map.entry(num).or_insert(0);
        //     *count += 1;
        //     if *count * 2 > nums.len() {
        //         return *num;
        //     }
        // }
        let mut majority_number = nums[0];
        let mut majority_count = 1;
        for num in nums.iter() {
            if *num == majority_number {
                majority_count += 1;
            } else {
                if majority_count > 0 {
                    majority_count -= 1;
                } else {
                    majority_count = 1;
                    majority_number = *num;
                }
            }
        }
        majority_number
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_majority_element() {
        let nums = vec![3,2,3];
        let k = Solution::majority_element(nums);
        assert_eq!(k, 3);
        let nums = vec![2,2,1,1,1,2,2];
        let k = Solution::majority_element(nums);
        assert_eq!(k, 2);

        let nums = vec![1,3,1,1,4,1,1,5,1,1,6,2,2];
        let k = Solution::majority_element(nums);
        assert_eq!(k, 1);

        let nums = vec![10,9,9,9,10];
        let k = Solution::majority_element(nums);
        assert_eq!(k, 9);
    }
}