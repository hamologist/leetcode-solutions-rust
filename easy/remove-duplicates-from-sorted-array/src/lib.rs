pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut current: usize = 1;

        let mut prev = nums[0];
        for &num in nums[1..].to_vec().iter() {
            if prev != num {
                nums[current] = num;
                current += 1;
            }
            prev = num;
        }

        return current as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 1, 2];
        let actual = Solution::remove_duplicates(&mut nums);

        assert_eq!(actual, 2);
        assert!(vec![1, 2]
            .iter()
            .enumerate()
            .all(|(index, &item)| nums[index] == item))
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual = Solution::remove_duplicates(&mut nums);

        assert_eq!(actual, 5);
        assert!(vec![0, 1, 2, 3, 4]
            .iter()
            .enumerate()
            .all(|(index, &item)| nums[index] == item))
    }
}
