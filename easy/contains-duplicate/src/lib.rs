pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut lookup = std::collections::HashSet::new();
        for num in nums {
            if lookup.insert(num) == false {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let actual = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(actual, true);
    }

    #[test]
    fn example_two() {
        let actual = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(actual, false);
    }

    #[test]
    fn example_three() {
        let actual = Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(actual, true);
    }
}
