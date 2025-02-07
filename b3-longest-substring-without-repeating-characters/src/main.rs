use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // let length = s.len();
        // if length == 0 {
        //     return 0;
        // }
        // let s_vec: Vec<char> = s.chars().collect();
        // let mut left_index = 0;
        // let mut right_index = 0;
        // let mut char_hashmap:HashMap<char, usize> = HashMap::new();
        // let mut max_length = 1;
        // char_hashmap.insert(s_vec[0], 0);
        // let mut next_char: char;
        // while left_index < length - 1 && right_index < length - 1 {
        //     next_char = s_vec[right_index+1];
        //     if let Some(&found_index) = char_hashmap.get(&next_char) {
        //         // char in index (right_index + 1) and found_index is the same
        //
        //         // reset hashmap from left_index to found_index
        //         for index in left_index..(found_index + 1) {
        //             char_hashmap.remove(&s_vec[index]);
        //         }
        //         // move left_index to found_index + 1
        //         left_index = found_index + 1;
        //     }
        //     right_index += 1;
        //     char_hashmap.insert(next_char, right_index);
        //     max_length = max_length.max(right_index - left_index + 1);
        // }
        // max_length as i32

        let length = s.len();
        if length == 0 {
            return 0;
        }
        let s_vec: Vec<u8> = s.bytes().collect();
        let mut left_index = 0;
        let mut right_index = 0;
        // save for all ASCII character, bc s consists of English letters, digits, symbols and spaces
        let mut char_vec:Vec<usize> = vec!(0; 128);
        let mut max_length = 1;
        char_vec[s_vec[0] as usize] = 1;
        let mut found_index = 0; // index from 1 to length

        while left_index < length - 1 && right_index < length - 1 {
            // println!("left_index: {left_index}, right_index: {right_index}, max_length: {max_length}");
            found_index = char_vec[s_vec[right_index+1] as usize];
            // println!("found_index: {found_index}");
            if found_index > 0 {
                // char in index (right_index + 1) and found_index - 1 is the same
                // reset char_vec from left_index to found_index
                for index in left_index..found_index {
                    char_vec[s_vec[index] as usize] = 0;
                }
                // move left_index to found_index
                left_index = found_index;
            }
            right_index += 1;
            char_vec[s_vec[right_index] as usize] = right_index + 1;
            max_length = max_length.max(right_index - left_index + 1);
        }
        max_length as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let s = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = String::from("bbbbb");
        assert_eq!(Solution::length_of_longest_substring(s), 1);

        let s = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = String::from("abba");
        assert_eq!(Solution::length_of_longest_substring(s), 2);
    }
}