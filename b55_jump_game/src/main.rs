struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        // let mut jumpables = vec![false; length];
        // jumpables[length - 1] = true;
        // for index in (0..(length-1)).rev() {
        //     for j in 1..(nums[index] + 1) {
        //         if jumpables[index + j as usize] {
        //             jumpables[index] = true;
        //             break;
        //         }
        //     }
        // }
        // jumpables[0]
        let mut max_jump_index = 0;
        let mut local_max;
        for index in 0..length {
            if index > max_jump_index {
                return false;
            } else {
                local_max = index + nums[index] as usize;
                if local_max > max_jump_index {
                    max_jump_index = local_max;
                }
            }
        }
        if max_jump_index >= length - 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        let nums = vec![2,3,1,1,4];
        let output = Solution::can_jump(nums);
        assert_eq!(output, true);
        
        let nums = vec![3,2,1,0,4];
        let output = Solution::can_jump(nums);
        assert_eq!(output, false);
    }
}