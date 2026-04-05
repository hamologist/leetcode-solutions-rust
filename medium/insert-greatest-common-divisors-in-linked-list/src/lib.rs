pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    return a;
}

impl Solution {
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        loop {
            let node = match current {
                None => break,
                Some(node) => node
            };
            let next = match node.next.take() {
                None => break,
                Some(next) => next
            };

            node.next = Some(Box::new(ListNode {
                val: gcd(node.val, next.val),
                next: Some(next)
            }));
            current = &mut node.next.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_node_from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current: &mut Option<Box<ListNode>> = &mut head; 

        for num in nums {
            current = match current {
                Some(current) => {
                    current.next = Some(Box::new(ListNode::new(num)));

                    &mut current.next
                }
                None => {
                    head = Some(Box::new(ListNode::new(num)));

                    &mut head
                },
            }
        }


        return head;
    }

    fn vec_from_list_node(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;

        loop {
            current = match current {
                None => break,
                Some(current) => {
                    result.push(current.val);

                    current.next
                }
            }
        }

        return result;
    }

    #[test]
    fn example_one() {
        let head = list_node_from_vec(vec![18, 6, 10, 3]);
        let result = Solution::insert_greatest_common_divisors(head);
        assert_eq!(vec_from_list_node(result), vec![18, 6, 6, 2, 10, 1, 3]);
    }

    #[test]
    fn example_two() {
        let head = list_node_from_vec(vec![7]);
        let result = Solution::insert_greatest_common_divisors(head);
        assert_eq!(vec_from_list_node(result), vec![7]);
    }
}
