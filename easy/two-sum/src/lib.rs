use std::collections::hash_map;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup = std::collections::HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            match lookup.entry(target - num) {
                hash_map::Entry::Vacant(_) => {
                    lookup.insert(num, i as i32);
                },
                hash_map::Entry::Occupied(entry) => {
                    return vec![*entry.get(), i as i32]
                }
            }
        }

        return vec![-1, -1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let actual = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(actual, vec![0, 1]);
    }

    #[test]
    fn example_two() {
        let actual = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(actual, vec![1, 2]);
    }

    #[test]
    fn example_three() {
        let actual = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(actual, vec![0, 1]);
    }
}
