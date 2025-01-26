struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let length = ratings.len();
        let mut left_candies = vec!(1;length);
        for index in 1..length {
            if ratings[index] > ratings[index-1] {
                left_candies[index] = left_candies[index-1] + 1;
            }
        }
        let mut current_right_candy = 1;
        let mut total_candies = left_candies[length-1];
        for index in (0..(length-1)).rev() {
            if ratings[index] > ratings[index+1] {
                current_right_candy += 1;
            } else {
                current_right_candy = 1;
                
            }
            total_candies += current_right_candy.max(left_candies[index]);
        }
        total_candies
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_candy() {
        let ratings = vec![1,0,2];
        let candy = Solution::candy(ratings);
        assert_eq!(candy, 5);

        let ratings = vec![1,2,2];
        let candy = Solution::candy(ratings);
        assert_eq!(candy, 4);
        
        let ratings = vec![1,3,2,2,1];
        let candy = Solution::candy(ratings);
        assert_eq!(candy, 7);

        let ratings = vec![1,6,10,8,7,3,2];
        let candy = Solution::candy(ratings);
        assert_eq!(candy, 18);
    }
}