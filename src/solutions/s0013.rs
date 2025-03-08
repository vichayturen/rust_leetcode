use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        let len = s.len();
        let mut res = 0;
        for i in 0..len-1 {
            if map.get(&s[i]) < map.get(&s[i + 1]) {
                res -= map.get(&s[i]).unwrap();
            } else {
                res += map.get(&s[i]).unwrap();
            }
        }
        res + map.get(&s[len - 1]).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::roman_to_int(String::from("III"));
        println!("{}", res);
    }
}
