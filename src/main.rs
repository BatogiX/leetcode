pub struct Solution;

impl Solution {
    #[must_use]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in &prices[1..] {
            if *price < min_price {
                min_price = *price;
                continue;
            }

            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }

        max_profit
    }
}

fn main() {
    assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    assert_eq!(12, Solution::max_profit(vec![8, 20, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![1]));
    assert_eq!(9, Solution::max_profit(vec![1, 10]));
}
