use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct RandomizedSet {
    position: HashMap<i32, usize>,
    values: Vec<i32>
}

impl RandomizedSet {
    fn new() -> Self {
        return RandomizedSet{
            position: HashMap::new(),
            values: Vec::new()
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let Some(_val_index) = self.position.get(&val) {
            false
        } else {
            // insert in the end and return true
            self.values.push(val);
            self.position.insert(val, self.values.len() - 1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        // fetch the last item and replace in position of this value
        if let Some(&val_index) = self.position.get(&val) {
            let last_val = self.values.pop().unwrap();
            if val_index < self.values.len() {
                self.values[val_index] = last_val;
                self.position.insert(last_val, val_index);
            }
            self.position.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        *self.values.choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let mut obj = RandomizedSet::new();
        // let ret_1: bool = obj.insert(3);
        // let ret_2: bool = obj.remove(2);
        // let ret_3: i32 = obj.get_random();
        // assert_eq!(ret_1, true);
        // assert_eq!(ret_2, false);
        // assert_eq!(ret_3, 3);
        obj.remove(0);
        obj.remove(0);
        obj.insert(0);
        obj.get_random();
        obj.remove(0);
        obj.insert(0);
    }
}