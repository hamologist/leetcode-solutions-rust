pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count: usize = 0;
        let nums_copy = nums.to_vec();

        for num in nums_copy {
            if num != val {
                nums[count] = num;
                count += 1;
            }
        }

        for i in count..nums.len() {
            nums[i] = -1;
        }

        return count as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![3, 2, 2, 3];
        let actual = Solution::remove_element(&mut nums, 3);

        assert_eq!(actual, 2);
        assert_eq!(nums, [2, 2, -1, -1]);
        assert!(vec![2, 2, -1, -1].iter().all(|item| nums.contains(item)));
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let actual = Solution::remove_element(&mut nums, 2);

        assert_eq!(actual, 5);
        assert!(vec![0, 1, 4, 0, 3, -1, -1, -1]
            .iter()
            .all(|item| nums.contains(item)));
    }
}
