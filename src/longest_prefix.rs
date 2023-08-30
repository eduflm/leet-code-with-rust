//Accept with bad memory and bad performance score

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut longest_prefix : String = "".to_owned();
    let mut current_index: usize = 0;

    let first_string : &String = &strs[0];

    while true {
        let current_letter = first_string.chars().nth(current_index);
        for i in 1..strs.len() {
            let current_string_letter = strs[i].to_owned().chars().nth(current_index);
            if current_string_letter != current_letter {
                return longest_prefix
            }
        }

        if (current_index >= first_string.len()) {
            return longest_prefix;
        }

        current_index += 1;
        longest_prefix.push(current_letter.unwrap_or_default());
    }

    return longest_prefix;
}

#[cfg(test)]
mod palindrome_tests {
    use super::*;

    #[test]
    fn longest_common_prefix_first() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl");
    }

    #[test]
    fn longest_common_prefix_second() {
        assert_eq!(longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]), "");
    }

    #[test]
    fn longest_common_prefix_third() {
        assert_eq!(longest_common_prefix(vec!["".to_string()]), "");
    }

    #[test]
    fn longest_common_prefix_fourth() {
        assert_eq!(longest_common_prefix(vec!["edu".to_string()]), "edu");
    }

    #[test]
    fn longest_common_prefix_fifth() {
        assert_eq!(longest_common_prefix(vec!["".to_string(), "".to_string()]), "");
    }
}
