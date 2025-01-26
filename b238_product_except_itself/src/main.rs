struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // let mut total_product: i32 = 1;
        // let mut total_zero = 0;
        // for num in nums.iter() {
        //     if *num != 0 {
        //         total_product *= *num;
        //     } else {
        //         total_zero += 1;
        //     }
        // }
        // let mut products: Vec<i32> = Vec::new();
        // for num in nums.iter() {
        //     if total_zero > 1 {
        //         // there are at least 2 zero
        //         products.push(0);
        //         continue;
        //     }
        //     if *num == 0 {
        //         products.push(total_product);
        //     } else {
        //         if total_zero == 0 {
        //             // there is no zero
        //             products.push(total_product / *num);
        //         } else {
        //             // there are other zero number
        //             products.push(0);
        //         }
        //     }
        // }
        // products

        let length = nums.len();
        let mut products = vec!(1;length);

        // a0, a1, ..., a(n-1)
        // each number ai = (a0 * a1 * ... * a(i-1)) * (a(i+1) * ... * a(n-1)
        // calculate left
        let mut left_product = 1;
        for index in 0..length {
            products[index] *= left_product;
            left_product *= nums[index];
        }

        let mut right_product = 1;
        for index in (0..length).rev() {
            products[index] *= right_product;
            right_product *= nums[index];
        }
        products
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let nums = vec![1,2,3,4];
        let products = Solution::product_except_self(nums);
        assert_eq!(products, vec![24,12,8,6]);

        let nums = vec![-1,1,0,-3,3];
        let products = Solution::product_except_self(nums);
        assert_eq!(products, vec![0,0,9,0,0]);
    }
}