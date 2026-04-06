pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            if num % 3 != 0 {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = Solution::minimum_operations(vec![1, 2, 3, 4]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = Solution::minimum_operations(vec![3, 6, 9]);
        assert_eq!(result, 0);
    }
}
