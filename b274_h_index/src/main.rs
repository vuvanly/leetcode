struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // let mut citations_cp = citations.clone();
        // citations_cp.sort_by(|a, b| b.cmp(a));
        // let length = citations_cp.len();
        // // println!("{:?}", citations_cp);
        // let mut h_index = 0;
        // for index in 0..length {
        //     if citations_cp[index] < (index as i32 + 1) {
        //         break;
        //     } else {
        //         h_index += 1;
        //     }
        // }
        // h_index
        let mut index_map = vec!(0;1001);
        for index in 0..(citations.len()) {
            index_map[citations[index] as usize] += 1;
        }
        let mut h_index = 0;
        for index in (0..1001).rev() {
            if index_map[index] > 0 {
                if h_index > index {
                    break;
                }
                if h_index + index_map[index] >= index {
                    h_index = index;
                    break;
                }
                h_index += index_map[index];
            }
        }
        h_index as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_h_index() {
        let citations = vec![3,0,6,1,5];
        let h_index = Solution::h_index(citations);
        assert_eq!(h_index, 3);

        let citations = vec![1];
        let h_index = Solution::h_index(citations);
        assert_eq!(h_index, 1);

        let citations = vec![1,1];
        let h_index = Solution::h_index(citations);
        assert_eq!(h_index, 1);
    }
}