struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(mut scores: Vec<i32>) -> Vec<String> {
        let mut ranks = {
            let mut ranks = Vec::with_capacity(scores.len());
            ranks.push("Gold Medal".to_owned());
            if scores.len() >= 2 {
                ranks.push("Silver Medal".to_owned());
            }
            if scores.len() >= 3 {
                ranks.push("Bronze Medal".to_owned());
            }
            for index in 3..scores.len() {
                ranks.push((index + 1).to_string());
            }
            ranks
        };

        let hm = {
            let mut hm = HashMap::with_capacity(scores.len());
            for (index, score) in scores.iter().enumerate() {
                hm.insert(*score, index);
            }
            hm
        };

        scores.sort_unstable();

        let mut answer = vec![String::with_capacity(0); scores.len()];
        for score in scores {
            answer[hm[&score]] = ranks.pop().unwrap();
        }

        answer
    }
}

fn main() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );

    assert_eq!(
        Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    );

    assert_eq!(Solution::find_relative_ranks(vec![10]), vec!["Gold Medal"]);
    assert_eq!(Solution::find_relative_ranks(vec![5, 10]), vec!["Silver Medal", "Gold Medal"]);
}
