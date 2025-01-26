struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // [1,2,3,0,0,0], m = 3, [2,5,6], n = 3
        // 
        let mut index_m: i32 = m -1;
        let mut index_n: i32 = n -1;
        let mut merge_index: i32 = m + n -1;
        while index_m >= 0 && index_n >= 0 {
            if nums1[index_m as usize] > nums2[index_n as usize] {
                nums1[merge_index as usize] = nums1[index_m as usize];
                index_m -= 1;
            } else {
                nums1[merge_index as usize] = nums2[index_n as usize];
                index_n -= 1;
            }
            merge_index -= 1;
        }

        if index_n >= 0 {
            for i in 0..(index_n+1) {
                nums1[i as usize] = nums2[i as usize]
            }
        }
    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let mut nums2 = vec![2,5,6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);

        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}