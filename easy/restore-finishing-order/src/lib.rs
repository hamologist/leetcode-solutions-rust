use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let friends_lookup: HashSet<i32> = HashSet::from_iter(friends);

        for racer_id in order {
            if friends_lookup.contains(&racer_id) {
                result.push(racer_id);
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
        let result = Solution::recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]);
        assert_eq!(result, vec![3, 1, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]);
        assert_eq!(result, vec![5, 2]);
    }
}
