pub struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec!();

        for num in 1..=n {
            if num % 3 == 0 && num % 5 == 0 {
                result.push(String::from("FizzBuzz"));
            } else if num % 3 == 0 {
                result.push(String::from("Fizz"));
            } else if num % 5 == 0 {
                result.push(String::from("Buzz"));
            } else {
                result.push(num.to_string());
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example_one() {
        let actual = Solution::fizz_buzz(3);
        assert_eq!(actual, vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn example_two() {
        let actual = Solution::fizz_buzz(5);
        assert_eq!(actual, vec!["1","2","Fizz","4","Buzz"]);
    }

    #[test]
    fn example_three() {
        let actual = Solution::fizz_buzz(15);
        assert_eq!(actual, vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]);
    }
}

