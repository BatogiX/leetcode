fn main() {
    let strs = vec!["a".to_string(), "ab".to_string()];
    println!("{}", longest_common_prefix(strs));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs_iter = strs.iter();
    let mut prefix = strs_iter.next().unwrap().as_str();

    for string in strs_iter {
        if prefix.is_empty() {
            break;
        }

        let (mut prefix_chrs, mut string_chrs) = (prefix.chars(), string.chars());
        for index in 0..prefix.len().max(string.len()) {
            let (prefix_chr_opt, string_chr_opt) = (prefix_chrs.next(), string_chrs.next());

            match (prefix_chr_opt, string_chr_opt) {
                (Some(prefix_chr), Some(string_chr)) => {
                    if prefix_chr != string_chr {
                        prefix = &prefix[..index];
                        break;
                    }
                    continue;
                }
                (Some(_), None) => {
                    prefix = &prefix[..index];
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }

    prefix.to_owned()
}
