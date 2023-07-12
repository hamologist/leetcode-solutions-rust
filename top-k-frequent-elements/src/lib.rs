pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut counter = std::collections::HashMap::<i32, i32>::new();
        let mut lookup: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        for &num in nums.iter() {
            counter.entry(num).and_modify(|num| *num += 1).or_insert(1);
        }

        for (&num, &count) in counter.iter() {
            lookup[count as usize].push(num);
        }

        let mut k_countdown = k;
        for nums in lookup.iter().rev() {
            for num in nums {
                result.push(*num);
                k_countdown -= 1;
                
                if k_countdown == 0 {
                    break;
                }
            }

            if k_countdown == 0 {
                break
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
        let actual = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(actual, vec![1, 2]);
    }

    #[test]
    fn example_two() {
        let actual = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(actual, vec![1]);
    }
}
