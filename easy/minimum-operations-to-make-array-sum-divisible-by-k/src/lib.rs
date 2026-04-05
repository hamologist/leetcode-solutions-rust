pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for num in nums {
            sum += num
        }

        return sum % k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = Solution::min_operations(vec![3, 9, 7], 5);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = Solution::min_operations(vec![4, 1, 3], 4);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_three() {
        let result = Solution::min_operations(vec![3, 2], 6);
        assert_eq!(result, 5);
    }
}
