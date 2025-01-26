fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut max_length = strs[0].len();
        for index in 1..(strs.len()) {
            let length = strs[index].len();
            max_length = length.min(max_length);
            for j in 0..max_length {
                if strs[index].as_bytes()[j] != strs[0].as_bytes()[j] {
                    max_length = j;
                    break;
                }
            }
        }
        String::from(strs[0].get(0..max_length).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        let lcprefix = Solution::longest_common_prefix(strs);
        assert_eq!(lcprefix, "fl");

        let strs = vec![String::from("dog"), String::from("racecar"), String::from("car")];
        let lcprefix = Solution::longest_common_prefix(strs);
        assert_eq!(lcprefix, "");
    }
}
