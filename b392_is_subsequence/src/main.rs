fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        if s.len() == 0 {
            return true;
        }
        // let vec_s: Vec<char> = s.chars().collect();
        // let vec_t: Vec<char> = t.chars().collect();
        // let mut s_index = 0;
        // let mut t_index = 0;
        // while t_index < vec_t.len() {
        //     if vec_t[t_index] == vec_s[s_index] {
        //         s_index += 1;
        //     }
        //     t_index += 1;
        //
        //     if s_index == vec_s.len() {
        //         return true;
        //     }
        // }

        let mut s_chars = s.chars();
        let mut s_char:Option<char> = s_chars.next();
        for t_char in t.chars() {
            if t_char == s_char.unwrap() {
                s_char = s_chars.next();
            }
            if s_char.is_none() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert_eq!(Solution::is_subsequence(s, t), true);

        // let s = String::from("axc");
        // let t = String::from("ahbgdc");
        // assert_eq!(Solution::is_subsequence(s, t), false);
    }
}