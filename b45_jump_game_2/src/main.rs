struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // let length = nums.len();
        // let mut jumpables = vec!(0;length);
        // let mut max_jump_index = 0;
        // let mut local_max;
        // let mut min_jump;
        // let mut jumpable_index = 0;
        // for index in 0..length {
        //     if index > max_jump_index {
        //         return 0;
        //     } else {
        //         local_max = index + nums[index] as usize;
        //         if local_max > max_jump_index {
        //             max_jump_index = local_max;
        //             min_jump = jumpables[index] + 1;

        //             // set value from jumpable_index to local_max = min_jump
        //             for j in (jumpable_index+1)..(local_max+1) {
        //                 if j < length {
        //                     jumpables[j] = min_jump;
        //                 }
        //             }

        //             jumpable_index = local_max;
        //         }
        //     }
        // }
        // if max_jump_index >= length - 1 {
        //     jumpables[length-1]
        // } else {
        //     0
        // }

        let length = nums.len();
        if length < 2 {
            return 0;
        }

        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..length - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == current_end {
                jumps += 1;
                current_end = farthest;
                if current_end >= length - 1 {
                    break;
                }
            }
        }

        jumps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump() {
        let nums = vec![2,3,1,1,4];
        let jump = Solution::jump(nums);
        assert_eq!(jump, 2);

        let nums = vec![1];
        let min_jump = Solution::jump(nums);
        assert_eq!(min_jump, 0);

        let nums = vec![7,0,9,6,9,6,1,7,9,0,1,2,9,0,3];
        let min_jump = Solution::jump(nums);
        assert_eq!(min_jump, 2);

        let nums = vec![3,4,3,2,5,4,3];
        let min_jump = Solution::jump(nums);
        assert_eq!(min_jump, 3);
    }
}