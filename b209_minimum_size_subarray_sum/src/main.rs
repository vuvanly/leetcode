fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut left_index = 0;
        let mut right_index = 0;
        let mut min_length = 0;
        let mut current_sum =  nums[0];
        while left_index < length && right_index < length {
            // println!("left_index: {left_index}, right_index: {right_index}, current_sum: {current_sum}");
            if current_sum >= target {
                if min_length == 0 || min_length > right_index - left_index + 1 {
                    min_length = right_index - left_index + 1;
                }
                // move both index to the right
                current_sum -= nums[left_index];
                left_index += 1;
            } else {
                // move right_index to the right to increase the sum
                right_index += 1;
                if right_index < length {
                    current_sum += nums[right_index];
                }
            }
        }
        min_length as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        let target = 7;
        let nums = vec![2,3,1,2,4,3];
        assert_eq!(Solution::min_sub_array_len(target, nums), 2);

        let target = 4;
        let nums = vec![1,4,4];
        assert_eq!(Solution::min_sub_array_len(target, nums), 1);

        let target = 11;
        let nums = vec![1,1,1,1,1,1,1,1];
        assert_eq!(Solution::min_sub_array_len(target, nums), 0);

        let target = 11;
        let nums = vec![1,2,3,4,5];
        assert_eq!(Solution::min_sub_array_len(target, nums), 3);
    }
}
