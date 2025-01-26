struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let ku = (k as usize) % length;
        // let nums_ori = nums.clone();
        // for index in 0..length {
        //     let replace_index: usize;
        //     if index >= ku {
        //         replace_index = index - ku;
        //     } else {
        //         replace_index = index + length - ku;
        //     }
        //     nums[index] = nums_ori[replace_index];
        // }

        if ku == 0 {
            return;
        }

        let mut start_index = 0;
        let mut prev_num = nums[0];
        let mut total_count = 0;
        let mut count_round = 0;
        while total_count < length {
            total_count += 1;
            count_round += 1;
            let next_index = (start_index + count_round*ku) % length;
            let tmp = nums[next_index];
            nums[next_index] = prev_num;
            prev_num = tmp;
            if next_index == start_index {
                start_index += 1;
                count_round = 0; // reset
                prev_num = nums[start_index];
            }
        }


        // if length % ku == 0 {
        //     for i in 0..ku {
        //         let mut prev_num = nums[i];
        //         for j in 1..(length/ku+1) {
        //             let next_index = (i + j*ku) % length;
        //             let tmp = nums[next_index];
        //             nums[next_index] = prev_num;
        //             prev_num = tmp;
        //         }
        //     }
        // } else {
        //     let mut prev_num = nums[0];
        //     for i in 0..length {
        //         let next_index = (i*ku) % length;
        //         let tmp = nums[next_index];
        //         nums[next_index] = prev_num;
        //         prev_num = tmp;
        //     }
        //     nums[0] = prev_num;
        // }
        // let mut saved_num = nums[0];
        // let mut next_saved_num:i32;
        // let mut current_index: usize = 0;
        // let mut replace_index: usize;
        // while count < length {
        //     replace_index = (current_index + ku ) % length;
        //     println!("current_index={},replace_index={}", current_index, replace_index);
        //     next_saved_num = nums[replace_index];
        //     nums[replace_index] = saved_num;
        //     saved_num = next_saved_num;
        //     current_index = replace_index;
        //     count += 1;
        //     println!("{:?}", nums);
        // }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);

        let mut nums = vec![-1,-100,3,99];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![3,99,-1,-100]);
    }
}