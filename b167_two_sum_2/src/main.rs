fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_index = 0;
        let mut right_index = numbers.len() - 1;
        let mut sum;
        while left_index < right_index {
            sum = numbers[left_index] + numbers[right_index];
            if sum == target {
                // found
                break;
            } else if sum > target {
                // decrease right
                right_index -= 1;
            } else {
                left_index += 1;
            }
        }
        vec![left_index as i32 + 1, right_index as i32 + 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2,7,11,15];
        let target = 9;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);

        let numbers = vec![2,3,4];
        let target = 6;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 3]);
    }
}
