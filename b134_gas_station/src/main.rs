struct Solution;

impl Solution {
    pub fn can_complete_circus(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let length = gas.len();
        let mut best_index = -1;
        let mut sum_to_now = 0;
        let mut remain;
        for index in 0..length {
            remain = gas[index] - cost[index];
            if sum_to_now < 0 {
                if remain > 0 {
                    best_index = index as i32;
                    sum_to_now = remain;
                } else {
                    // remain < 0 => reset
                    best_index = -1;
                    sum_to_now = 0;
                }
            } else {
                // > 0
                if sum_to_now + remain >= 0 {
                    // continue
                    sum_to_now += remain;
                    if best_index == -1 {
                        best_index = index as i32;
                    }
                } else {
                    // => remain < 0, sum_to_now >=0, sum_to_now + remain < 0 => reset
                    best_index = -1;
                    sum_to_now = 0;
                }
            }
        }

        if best_index == -1 {
            return -1;
        }

        // check again to make sure best_index is the right one
        let mut remain = 0;
        let mut current_index;
        let mut remain_i;
        for index in 0..length {
            current_index = (index + best_index as usize) % length;
            remain_i = gas[current_index] - cost[current_index];
            if remain + remain_i < 0 {
                return -1;
            }
            remain += remain_i;
        }

        best_index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_complete_circus() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];
        let output = Solution::can_complete_circus(gas, cost);
        assert_eq!(output, 3);

        let gas = vec![2,3,4];
        let cost = vec![3,4,3];
        let output = Solution::can_complete_circus(gas, cost);
        assert_eq!(output, -1);

        let gas = vec![5,1,2,3,4];
        let cost = vec![4,4,1,5,1];
        let output = Solution::can_complete_circus(gas, cost);
        assert_eq!(output, 4);
    }
}