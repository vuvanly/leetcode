struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k:usize = 0;
        let length = nums.len();
        for index in 0..length {
            if nums[index] != val {
                nums[k] = nums[index];
                k+=1;
            }
        }
        
        for index in k..length {
            nums[index] = -1;
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3,2,2,3];
        let val = 3;
        let k = Solution::remove_element(&mut nums, val);
        nums.sort_by(|a, b| b.cmp(a));
        assert_eq!(k, 2);
        assert_eq!(nums, vec![2,2,-1,-1]);

        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let k = Solution::remove_element(&mut nums, val);
        nums.sort_by(|a, b| b.cmp(a));
        assert_eq!(k, 5);
        assert_eq!(nums, vec![4,3,1,0,0,-1,-1,-1])
    }
}

