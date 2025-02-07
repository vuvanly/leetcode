fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums;
        let length = sorted_nums.len();
        sorted_nums.sort();
        // println!("sorted nums: {:?}", sorted_nums);
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut left_index;
        let mut right_index;
        let mut sum;
        for index in 0..(length - 2) {
            if sorted_nums[index] > 0 {
                // make no sense to continue bc the other numbers will greater than 0
                break;
            }
            if index > 0 && sorted_nums[index] == sorted_nums[index - 1] {
                // skip because it will have the same combination as previous one
                continue;
            }
            left_index = index + 1;
            right_index = length - 1;
            while left_index < right_index  {
                if left_index > index + 1 && sorted_nums[left_index] == sorted_nums[left_index - 1] {
                    // skip because it will have the same combination as previous one
                    left_index += 1;
                    continue;
                }

                if right_index < length - 1 && sorted_nums[right_index] == sorted_nums[right_index + 1] {
                    // skip because it will have the same combination as previous one
                    right_index -= 1;
                    continue;
                }

                sum = sorted_nums[left_index] + sorted_nums[right_index] + sorted_nums[index];
                if sum == 0 {
                    // found
                    result.push(vec![sorted_nums[index], sorted_nums[left_index], sorted_nums[right_index]]);
                    left_index += 1;
                    right_index -= 1;
                } else if sum > 0 {
                    // move right index to left so the sum will be smaller
                    if sum > sorted_nums[right_index] {
                        break;
                    }
                    right_index -= 1;
                } else {
                    // move left index to right so the sum will be greater
                    left_index += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1,0,1,2,-1,-4];
        assert_eq!(Solution::three_sum(nums), vec![vec![-1,-1,2], vec![-1,0,1]]);

        let nums = vec![0, 1, 1];
        assert_eq!(Solution::three_sum(nums), Vec::<Vec<i32>>::new());

        let nums = vec![0, 0, 0];
        assert_eq!(Solution::three_sum(nums), vec![vec![0, 0, 0]]);

        let nums = vec![-2,0,0,2,2];
        assert_eq!(Solution::three_sum(nums), vec![vec![-2, 0, 2]]);

        let nums = vec![-1,0,1,2,-1,-4,-2,-3,3,0,4];
        assert_eq!(Solution::three_sum(nums), vec![vec![-4,0,4],vec![-4,1,3],vec![-3,-1,4],vec![-3,0,3],vec![-3,1,2],vec![-2,-1,3],vec![-2,0,2],vec![-1,-1,2],vec![-1,0,1]]);
    }
}