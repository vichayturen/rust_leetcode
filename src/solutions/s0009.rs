pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut vec: Vec<i32> = Vec::new();
        let mut x = x;
        while x > 0 {
            let yushu = x % 10;
            vec.push(yushu);
            x = x / 10;
        }
        for i in 0..vec.len()/2 {
            if vec[i] != vec[vec.len()-1-i] {
                return false;
            }
        }
        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::is_palindrome(122);
        println!("{}", res);
    }
}
