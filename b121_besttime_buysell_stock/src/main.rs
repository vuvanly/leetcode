struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut max_profit = 0;
        for index in 0..(prices.len()-1) {
            let diff = prices[index+1] - prices[index];
            if diff >= 0 {
                // increase
                profit += diff;
            } else {
                // decrease
                profit = profit + diff;
                if profit < 0 {
                    // minus -> reset to 0
                    profit = 0;
                }
            }

            if profit > max_profit {
                max_profit = profit;
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_profit() {
        let prices = vec![7,1,5,3,6,4];
        let k = Solution::max_profit(prices);
        assert_eq!(k, 5);
    }
}