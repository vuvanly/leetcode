impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // [1,2,3,0,0,0], m = 3, [2,5,6], n = 3
        // 
        let mut indexM: usize = m -1;
        let mut indexN: usize = n -1;
        let mut mergeIndex: usize = m + n -1;
        while (indexM >= 0 && indexN >= 0) {
            if (nums1[indexM] > nums2[indexN]) {
                nums1[mergeIndex] = nums1[indexM];
                indexM -= 1;
            } else {
                nums1[mergeIndex] = nums2[indexN];
                indexN -= 1;
            }
            mergeIndex -= 1;
        }

        if (indexN >= 0) {
            for i in 0..indexN {
                nums1[i] = nums2[i]
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