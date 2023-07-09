pub struct Solution {}

impl Solution {
    pub fn majority_element_using_hash_map(nums: Vec<i32>) -> i32 {
        let target_count = nums.len() / 2;
        let mut lookup = std::collections::HashMap::new();

        for num in nums {
            lookup.entry(num).and_modify(|val| *val += 1).or_insert(1);

            if lookup[&num] > target_count {
                return num;
            }
        }

        return 0;
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut votes = 0;
        for num in nums {
            if num == candidate {
                votes += 1;
            } else {
                votes -= 1;

                if votes < 0 {
                    candidate = num;
                    votes = 1;
                }
            }
        }

        return candidate;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn example_one() {
        let actual = Solution::majority_element(vec![3, 2, 3]);
        assert_eq!(3, actual);
    }

    #[test]
    fn example_two() {
        let actual = Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
        assert_eq!(2, actual);
    }

    mod using_hash_map {
        use crate::Solution;
        #[test]
        fn example_one() {
            let actual = Solution::majority_element_using_hash_map(vec![3, 2, 3]);
            assert_eq!(3, actual);
        }

        #[test]
        fn example_two() {
            let actual = Solution::majority_element_using_hash_map(vec![2, 2, 1, 1, 1, 2, 2]);
            assert_eq!(2, actual);
        }
    }
}
