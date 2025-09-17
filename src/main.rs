struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let keyboard = {
            let mut keyboard = HashMap::new();
            let rows = [
                vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
                vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
                vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
            ];

            for (index, row) in rows.into_iter().enumerate() {
                for char in row {
                    keyboard.insert(char, index as i8);
                    keyboard.insert(char.to_ascii_uppercase(), index as i8);
                }
            }

            keyboard
        };

        let mut answer = vec![];
        'outer: for word in words {
            let mut word_chars = word.chars();
            let mut previous_row_number = keyboard[&word_chars.next().unwrap()];

            for char in word_chars {
                let row_number = keyboard[&char];

                if row_number != previous_row_number {
                    continue 'outer;
                }
                previous_row_number = row_number;
            }

            answer.push(word);
        }

        answer
    }
}

fn main() {
    println!("a: {}, A: {}", 'a' as u32, 'A' as u32);
    assert_eq!(
        Solution::find_words(vec![
            "Hello".to_owned(),
            "Alaska".to_owned(),
            "Dad".to_owned(),
            "Peace".to_owned()
        ]),
        vec!["Alaska", "Dad"]
    );

    assert_eq!(
        Solution::find_words(vec!["omk".to_owned()]),
        Vec::<String>::new()
    );

    assert_eq!(
        Solution::find_words(vec!["adsdf".to_owned(), "sfd".to_owned()]),
        vec!["adsdf", "sfd"]
    );

    assert_eq!(
        Solution::find_words(vec!["a".to_owned(), "b".to_owned()]),
        vec!["a", "b"]
    );
}
