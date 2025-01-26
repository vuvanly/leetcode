fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // let num_rows = num_rows as usize;
        // if num_rows == 1 {
        //     return s;
        // }
        // let length = s.len();
        // let zz_block_count: usize = 2 * num_rows - 2;
        // let mut count_each_rows = vec!(0;num_rows);
        // let k = length % zz_block_count;
        // let q = length / zz_block_count;
        // for index in 0..num_rows {
        //     if index == 0 || index == num_rows -1 {
        //         count_each_rows[index] = q;
        //         if k >= (index + 1) {
        //             count_each_rows[index] += 1;
        //         }
        //     } else if index ==1 && index < num_rows - 1 {
        //         count_each_rows[index] = 2 * q;
        //         if k >= 2 {
        //             count_each_rows[index] += 1;
        //         }
        //     } else {
        //         // between index 0 and n-1
        //         count_each_rows[index] = 2 * q;
        //         if k > zz_block_count - index {
        //             count_each_rows[index] += 2;
        //         } else if k > index {
        //             count_each_rows[index] += 1;
        //         }
        //     }
        // }
        // let cvec: Vec<char> = s.chars().collect();
        // let mut czz_vec = cvec.clone();
        // let mut count_previos_line = 0;
        // let mut ori_index;
        // let mut zz_index;
        // // println!("First row");
        // for index in 0..num_rows {
        //     if index > 0 {
        //         count_previos_line += count_each_rows[index - 1];
        //     }
        //     let between_step = match index {
        //         index if index == 0 || index == num_rows - 1 => 1,
        //         _ => 2
        //     };
        //     for col in 0..(q + 1) {
        //         ori_index = index + col * zz_block_count;
        //         zz_index = count_previos_line + between_step * col;
        //         // println!("ori_index={}, zz_index={}", ori_index, zz_index);
        //         if ori_index < length && zz_index < length {
        //             czz_vec[zz_index] = cvec[ori_index];
        //         }
        //     }
        // }
        // // println!("Second row");
        // count_previos_line = 0;
        // let mut row;
        // for index in (num_rows..zz_block_count).rev() {
        //     row = zz_block_count - index;
        //     count_previos_line += count_each_rows[row-1];
        //     for col in 0..(q + 1) {
        //         ori_index = index + col * zz_block_count;
        //         zz_index = count_previos_line + 2*col + 1;
        //         // println!("ori_index={}, zz_index={}", ori_index, zz_index);
        //         if ori_index < length && zz_index < length {
        //             czz_vec[zz_index] = cvec[ori_index];
        //         }
        //     }
        // }
        // // println!("{:?}", count_each_rows);
        // czz_vec.into_iter().collect()
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }
        let length = s.len();
        let zz_block_count: usize = 2 * num_rows - 2;
        let q = length / zz_block_count;
        let cvec: Vec<char> = s.chars().collect();
        let mut czz_vec = cvec.clone();
        let mut first_index;
        let mut second_index;
        let mut zz_index = 0;
        for index in 0..num_rows {
            for col in 0..(q+1) {
                first_index = index + col * zz_block_count;
                second_index = (zz_block_count - index) + col * zz_block_count;
                if first_index < length {
                    czz_vec[zz_index] = cvec[first_index];
                    zz_index += 1;
                }
                if second_index < length
                    && second_index != first_index
                    && second_index != first_index + zz_block_count {
                    czz_vec[zz_index] = cvec[second_index];
                    zz_index += 1;
                }
            }
        }
        czz_vec.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        let convert_s = Solution::convert(s, num_rows);
        assert_eq!(convert_s, "PINALSIGYAHRPI");

        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        let convert_s = Solution::convert(s, num_rows);
        assert_eq!(convert_s, "PAHNAPLSIIGYIR");

        let s = String::from("A");
        let num_rows = 1;
        let convert_s = Solution::convert(s, num_rows);
        assert_eq!(convert_s, "A");
    }
}