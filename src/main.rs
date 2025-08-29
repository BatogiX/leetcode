fn main() {
    assert_eq!(vec![1, 2, 4], plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], plus_one(vec![4, 3, 2, 1]));
    assert_eq!(vec![1, 0], plus_one(vec![9]));
}

#[must_use]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut answer = digits.clone();
    for (i, digit) in digits.into_iter().enumerate().rev() {
        if digit == 9 {
            answer[i] = 0;
        } else {
            answer[i] += 1;
            return answer;
        }
    }

    answer.insert(0, 1);
    answer
}
