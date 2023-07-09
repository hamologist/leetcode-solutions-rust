use std::collections::hash_map;

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut lookup = std::collections::HashMap::new();

        for char in s.chars() {
            lookup
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for char in t.chars() {
            match lookup.entry(char) {
                hash_map::Entry::Vacant(_) => return false,
                hash_map::Entry::Occupied(entry) => {
                    let count = entry.into_mut();

                    if *count == 0 {
                        return false;
                    }
                    *count -= 1;
                }
            };
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let actual = Solution::is_anagram(String::from("anagram"), String::from("nagaram"));
        assert_eq!(actual, true);
    }

    #[test]
    fn example_two() {
        let actual = Solution::is_anagram(String::from("rat"), String::from("car"));
        assert_eq!(actual, false);
    }
}
