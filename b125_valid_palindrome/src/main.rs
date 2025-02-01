fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let vec:Vec<char> = s.chars().collect();
        let mut left_index = 0;
        let mut right_index = vec.len() - 1;
        let mut left_char;
        let mut right_char;
        while left_index < right_index {
            // check left
            left_char = vec[left_index].to_ascii_lowercase();
            if !left_char.is_alphanumeric() {
                left_index += 1;
                continue;
            }

            // check right
            right_char = vec[right_index].to_ascii_lowercase();
            if !right_char.is_alphanumeric() {
                right_index -= 1;
                continue;
            }

            // compare left and right
            if left_char != right_char {
                return false
            } else {
                left_index += 1;
                right_index -= 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert_eq!(Solution::is_palindrome(s), true);

        let s = String::from("race a car");
        assert_eq!(Solution::is_palindrome(s), false);

        let s = String::from(" ");
        assert_eq!(Solution::is_palindrome(s), true);
    }
}