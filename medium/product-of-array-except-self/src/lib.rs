pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        let mut prefix = 1;
        for (i, _) in nums.iter().enumerate() {
            result[i] = prefix;
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for (i, _) in nums.iter().enumerate().rev() {
            result[i] *= postfix;
            postfix *= nums[i];
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let actual = Solution::product_except_self(vec![1,2,3,4]);
        assert_eq!(actual, [24,12,8,6]);
    }

    #[test]
    fn example_two() {
        let actual = Solution::product_except_self(vec![-1,1,0,-3,3]);
        assert_eq!(actual, [0,0,9,0,0]);
    }
}

