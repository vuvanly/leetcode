fn main() {
    println!("Hello, world!");
}


struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut strs: Vec<String> = Vec::new();
        let mut previous_is_space = true;
        let mut word = String::new();
        for c in s.chars() {
            if c != ' ' {
                // not space
                previous_is_space = false;
                word.push(c);
            } else {
                // space
                if !previous_is_space {
                    // end word
                    strs.push(word);
                    word = String::new();
                }
                previous_is_space = true;
            }
        }
        println!("strs: {:?}", strs);
        if word.len() > 0 {
            strs.push(word);
        }
        println!("strs: {:?}", strs);
        let mut rev = String::new();
        for index in (0..strs.len()).rev() {
            rev.push_str(&strs[index]);
            if index > 0 {
                rev.push(' ');
            }
        }
        rev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let s = String::from("the sky is blue");
        let r = Solution::reverse_words(s);
        assert_eq!(r, "blue is sky the");

        let s = String::from("  hello world  ");
        let r = Solution::reverse_words(s);
        assert_eq!(r, "world hello");

        let s = String::from("a good   example");
        let r = Solution::reverse_words(s);
        assert_eq!(r, "example good a");
    }
}