fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut begin_last_word = false;
        let mut length = 0;
        for c in s.chars().rev() {
            if begin_last_word && c == ' ' {
                return length;
            }
            if c != ' ' {
                begin_last_word = true;
                length += 1;
            }
        }
        length
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        let s = String::from("Hello World");
        let l = Solution::length_of_last_word(s);
        assert_eq!(l, 5);

        let s = String::from("   fly me   to   the moon  ");
        let l = Solution::length_of_last_word(s);
        assert_eq!(l, 4);

        let s = String::from("luffy is still joyboy");
        let l = Solution::length_of_last_word(s);
        assert_eq!(l, 6);

        let s = String::from("a");
        let l = Solution::length_of_last_word(s);
        assert_eq!(l, 1);
    }
}
