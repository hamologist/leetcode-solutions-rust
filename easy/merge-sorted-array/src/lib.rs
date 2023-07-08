pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let nums1_copy = nums1[..m as usize].to_vec();
        let mut current_m: usize = 0;
        let mut current_n: usize = 0;

        while (current_m as i32) < m && (current_n as i32) < n {
            let num1 = nums1_copy[current_m];
            let num2 = nums2[current_n];

            if num1 <= num2 {
                nums1[current_m + current_n] = num1;
                current_m += 1;
            } else {
                nums1[current_m + current_n] = num2;
                current_n += 1;
            }
        }
        while (current_m as i32) < m {
            nums1[current_m + current_n] = nums1_copy[current_m];
            current_m += 1;
        }
        while (current_n as i32) < n {
            nums1[current_m + current_n] = nums2[current_n];
            current_n += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_two() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example_three() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, vec![1]);
    }
}
