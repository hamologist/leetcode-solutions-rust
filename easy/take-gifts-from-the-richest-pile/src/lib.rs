use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gift_heap = BinaryHeap::from_iter(gifts);
        for _ in 0..k {
            if let Some(current) = gift_heap.pop() {
                gift_heap.push((current as f64).sqrt().floor() as i32)
            } else {
                return 0;
            }
        }

        let mut result = 0;
        for gift in gift_heap {
            result += gift as i64;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4);
        assert_eq!(result, 29);
    }

    #[test]
    fn example_two() {
        let result = Solution::pick_gifts(vec![1, 1, 1, 1], 4);
        assert_eq!(result, 4);
    }
}
