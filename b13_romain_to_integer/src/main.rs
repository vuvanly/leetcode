struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // romain to int
        // Symbol       Value
        // I             1
        // V             5
        // X             10
        // L             50
        // C             100
        // D             500
        // M             1000
        // I can be placed before V (5) and X (10) to make 4 and 9.
        // X can be placed before L (50) and C (100) to make 40 and 90.
        // C can be placed before D (500) and M (1000) to make 400 and 900.
        let mut number = 0;
        let mut current_number;
        let mut previos_number = 0;
        for c in s.chars().rev() {
            println!("{}", c);
            current_number = Self::roman_char_to_int(c);
            if current_number >= previos_number {
                // plus
                number += current_number;
            } else {
                // minus, e.x. IV
                number -= current_number;
            }
            previos_number = current_number;
        }
        number
    }

    fn roman_char_to_int(c: char) -> i32 {
        return match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let s = String::from("MCMXCIV");
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 1994);

        let s = String::from("III");
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 3);

        let s = String::from("LVIII");
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 58);
    }
}

fn main() {}