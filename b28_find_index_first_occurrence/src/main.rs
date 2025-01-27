fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // if needle.len() > haystack.len() {
        //     return -1;
        // }
        // let haystack_vec: Vec<char> = haystack.chars().collect();
        // let needle_vec: Vec<char> = needle.chars().collect();
        // for h_index in 0..(haystack.len() - needle.len()) {
        //     for n_index in 0..needle.len() {
        //         if haystack_vec[h_index + n_index] != needle_vec[n_index] {
        //             break;
        //         }
        //         if n_index == needle.len() - 1 {
        //             return h_index as i32;
        //         }
        //     }
        // }

        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_vec: Vec<char> = needle.chars().collect();
        let needle_len = needle_vec.len();
        let haystack_len = haystack_vec.len();
        if haystack_len < needle_len {
            return -1;
        }
        // use Knuth-Morris-Pratt-Algorithms
        // calculate Rand table
        let needle_rand = Self::get_rand_table(&needle_vec);
        let mut h_index = 0;
        let mut n_index = 0;
        let mut rand_value: usize;
        while h_index < haystack_len && n_index < needle_len {
            if haystack_vec[h_index] == needle_vec[n_index] {
                h_index += 1;
                n_index += 1;
                if n_index == needle_len {
                    return (h_index - n_index) as i32;
                }
            } else {
                // diff
                if n_index == 0 {
                    n_index = 0;
                    h_index += 1;
                } else {
                    rand_value = needle_rand[n_index - 1];
                    h_index = h_index - (n_index - rand_value) + 1;
                    n_index = rand_value;
                }
            }
        }
        -1
    }

    fn get_rand_table(needle_vec: &Vec<char>) -> Vec<usize> {
        let mut needle_rand = vec!(0; needle_vec.len());
        for index in 1..needle_vec.len() {
            if needle_vec[index] == needle_vec[0+needle_rand[index-1]] && needle_rand[index-1] * 2 < index {
                needle_rand[index] = needle_rand[index-1] + 1;
            } else if needle_vec[index] == needle_vec[0] {
                needle_rand[index] = 1;
            } else {
                needle_rand[index] = 0;
            }
        }
        needle_rand
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, 0);

        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, -1);

        let haystack = String::from("hello");
        let needle = String::from("ll");
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, 2);

        let haystack = String::from("babba");
        let needle = String::from("bbb");
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, -1);

        let haystack = String::from("babbbbbabb");
        let needle = String::from("bbab");
        let index = Solution::str_str(haystack, needle);
        assert_eq!(index, 5);
    }

    #[test]
    fn test_rand_table() {
        let s = String::from("ababcabab");
        let rand_table = Solution::get_rand_table(&s.chars().collect());
        assert_eq!(rand_table, vec![0, 0, 1, 2, 0, 1, 2, 3, 4]);

        let s = String::from("bbb");
        let rand_table = Solution::get_rand_table(&s.chars().collect());
        assert_eq!(rand_table, vec![0, 1, 1]);

        let s = String::from("bbab");
        let rand_table = Solution::get_rand_table(&s.chars().collect());
        assert_eq!(rand_table, vec![0, 1, 0, 1]);
    }
}