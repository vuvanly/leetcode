fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let length = height.len();
        let mut max_area = (length - 1) as i32 * height[0].min(height[length - 1]);
        let mut left_index = 0;
        let mut right_index= length - 1;
        while left_index < right_index {
            // move the index with less value to find the next max possible value, because there is no meaning to move the index the greater value
            if height[left_index] <= height[right_index] {
                left_index += 1;
                if height[left_index] > height[left_index - 1] {
                    max_area = max_area.max((right_index - left_index) as i32 * height[right_index].min(height[left_index]));
                }

            } else {
                right_index -= 1;
                if height[right_index] > height[right_index + 1] {
                    max_area = max_area.max((right_index - left_index) as i32 * height[right_index].min(height[left_index]));
                }
            }

        }
        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(Solution::max_area(height), 49);

        let height = vec![1,1];
        assert_eq!(Solution::max_area(height), 1);

        let height = vec![1,2,3,4,5,25,24,3,4];
        assert_eq!(Solution::max_area(height), 24);

        let height = vec![8,7,2,1];
        assert_eq!(Solution::max_area(height), 7);
    }
}
