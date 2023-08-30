pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let x_vec : Vec<i32> = x.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as i32)
    .collect();

    let mut start: usize = 0;
    let mut end : usize = x_vec.len() as usize;
    let middle = end / 2;

    end -= 1;

    println!("{:?}", middle);

    while start < middle {
        if x_vec[start] != x_vec[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}


#[cfg(test)]  // This attribute tells Rust to compile the tests only when running tests.
mod palindrome_tests {
    use super::*;  // Import symbols from the outer module.

    #[test]
    fn is_palindrome_first() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn is_palindrome_second() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn is_palindrome_third() {
        assert_eq!(is_palindrome(10), false);
    }
}
