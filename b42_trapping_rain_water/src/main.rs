struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let length = height.len();
        let mut left_max = height.clone();
        let mut previous_max = height[0];
        // go from left to right
        for index in 1..length {
            if height[index] <= previous_max {
                left_max[index] = previous_max;
            } else {
                // new peak
                previous_max = height[index];
            }
        }

        // go from right to left
        previous_max = height[length - 1];
        let mut trap = 0;
        for index in (0..length).rev() {
            if height[index] > previous_max {
                previous_max = height[index];
            }
            trap += previous_max.min(left_max[index]) - height[index];
        }
        trap
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trap() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let trap = Solution::trap(height);
        assert_eq!(trap, 6);

        let height = vec![4,2,0,3,2,5];
        let trap = Solution::trap(height);
        assert_eq!(trap, 9);
    }
}