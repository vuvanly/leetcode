// fn main() {
//     println!("Hello, world!");
// }
// Symbol	Value
// I	1
// V	5
// X	10
// L	50
// C	100
// D	500
// M	1000

struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut s = String::new();
        let values = vec![1000, 500, 100, 50, 10, 5, 1];
        let mut num_m = num;
        let mut k;
        for index in 0..7 {
            let value = values[index];
            if num_m == 0 {
                break;
            }
            if [1000, 100, 10, 1].contains(&value) {
                // 1000, 100, 10
                k = num_m / value;
                num_m = num_m % value;
                for _index in 0..k {
                    s.push(Self::int_to_char(value));
                }
                if num_m > 1 && num_m >= value / 10 * 9 {
                    s.push(Self::int_to_char(value/10));
                    s.push(Self::int_to_char(value));
                    num_m -= value / 10 * 9;
                }
            } else {
                // 500, 50, 5
                if num_m >= value {
                    s.push(Self::int_to_char(value));
                    num_m -= value;
                }
                if num_m >= (value / 5 * 4) {
                    s.push(Self::int_to_char(value/5));
                    s.push(Self::int_to_char(value));
                    num_m -= value / 5 * 4;
                }
            }
            // println!("num_m: {}", num_m);
        }
        s
    }

    fn int_to_char(num: i32) -> char {
        return match num {
            1000 => 'M',
            500 => 'D',
            100 => 'C',
            50 => 'L',
            10 => 'X',
            5 => 'V',
            1 => 'I',
            _ => 'I'
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        // let num = 3749;
        // let s = Solution::int_to_roman(num);
        // assert_eq!(s, String::from("MMMDCCXLIX"));

        let num = 58;
        let s = Solution::int_to_roman(num);
        assert_eq!(s, String::from("LVIII"));

        // let num = 1994;
        // let s = Solution::int_to_roman(num);
        // assert_eq!(s, String::from("MCMXCIV"));
    }
}

