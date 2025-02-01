fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width= max_width as usize;
        let length = words.len();
        let mut current_index = 0;
        let mut current_row_vec: Vec<String> = Vec::new();
        let mut current_row_words_length = 0;
        let mut justify_vec: Vec<String> = Vec::new();
        let mut row_string;
        while current_index < length {
            let word: &String = &words[current_index];
            println!("CurrentIndex: {current_index}, word: {:?}", word);
            if current_row_words_length + current_row_vec.len() + word.len() <= max_width {
                println!("same row");
                // still same row
                current_row_vec.push(word.clone());
                current_row_words_length += word.len();
            } else {
                // go to next row
                println!("next row, current_row_vec: {:?}", current_row_vec);
                // add to justify
                row_string = String::new();
                if current_row_vec.len() == 1 {
                    // only 1 words
                    row_string.push_str(&*current_row_vec[0]);
                    for _j in 0..(max_width - current_row_words_length) {
                        row_string.push(' ');
                    }
                } else {
                    // at least 2 words
                    let min_spaces = (max_width - current_row_words_length) / (current_row_vec.len() - 1);
                    let k = (max_width - current_row_words_length) % (current_row_vec.len() - 1); // first k words has min_spaces + 1 space, the rest min_spaces space
                    for i in 0..current_row_vec.len() {
                        row_string.push_str(&*current_row_vec[i]);
                        // println!("row_string: {:?}, current_word: {:?}", row_string, &*current_row_vec[0]);
                        if i < k {
                            for _j in 0..(min_spaces + 1) {
                                row_string.push(' ');
                            }
                        } else if i < current_row_vec.len() - 1 {
                            for _j in 0..min_spaces {
                                row_string.push(' ');
                            }
                        }
                    }
                }
                justify_vec.push(row_string);
                // and reset
                current_row_vec = Vec::new();
                current_row_vec.push(word.clone());
                current_row_words_length = word.len();
            }

            if current_index == length - 1 {
                println!("last index");
                row_string = String::new();
                // last index
                for i in 0..(current_row_vec.len()) {
                    row_string.push_str(&*current_row_vec[i]);
                    if i < current_row_vec.len() - 1 {
                        row_string.push(' ');
                    }
                }
                println!("current_row_words_length: {current_row_words_length}, len: {}", current_row_vec.len());
                for _j in 0..(max_width + 1 - current_row_words_length - current_row_vec.len()) {
                    row_string.push(' '); // add space at the end
                }
                justify_vec.push(row_string);
            }

            current_index += 1;
        }
        justify_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_justify() {
        let words = vec!["This", "is", "an", "example", "of", "text", "justification."].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let max_width = 16;
        let full_justifies = Solution::full_justify(words, max_width);
        assert_eq!(full_justifies, vec![
            "This    is    an",
            "example  of text",
            "justification.  "
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>());


        let words = vec!["What","must","be","acknowledgment","shall","be"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let max_width = 16;
        let full_justifies = Solution::full_justify(words, max_width);
        assert_eq!(full_justifies, vec![
            "What   must   be",
            "acknowledgment  ",
            "shall be        "
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>());

        let words = vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let max_width = 20;
        let full_justifies = Solution::full_justify(words, max_width);
        assert_eq!(full_justifies, vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>());


        let words = vec!["ask","not","what","your","country","can","do","for","you","ask","what","you","can","do","for","your","country"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let max_width = 16;
        let full_justifies = Solution::full_justify(words, max_width);
        assert_eq!(full_justifies, vec![
            "ask   not   what",
            "your country can",
            "do  for  you ask",
            "what  you can do",
            "for your country"
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>());
    }
}