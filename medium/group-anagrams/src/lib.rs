pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_counter = std::collections::HashMap::<[i32; 26], Vec<String>>::new();

        for str in strs {
            let mut lookup = [0; 26];

            for char in str.chars() {
                lookup[char as usize - 97] += 1;
            }

            anagram_counter
                .entry(lookup)
                .and_modify(|count| count.push(str.clone()))
                .or_insert(vec![str.clone()]);
        }

        let mut result: Vec<Vec<String>> = vec![];
        for value in anagram_counter.values() {
            result.push(value.clone())
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(expect: Vec<Vec<String>>, actual: Vec<Vec<String>>) -> bool {
        let actual: Vec<Vec<String>> = actual
            .iter()
            .map(|group| {
                let mut group = group.clone();
                group.sort();

                group
            })
            .collect();

        return expect.iter().all(|expect_group| {
            let mut group = expect_group.clone();
            group.sort();

            actual.contains(&group)
        });
    }

    fn generate_strs(strs: Vec<&str>) -> Vec<String> {
        strs.iter().map(|&str| str.to_string()).collect()
    }

    fn generate_expected(groups: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        groups
            .iter()
            .map(|group| group.iter().map(|&s| s.to_string()).collect())
            .collect()
    }

    #[test]
    fn example_one() {
        let actual = Solution::group_anagrams(generate_strs(vec![
            "eat", "tea", "tan", "ate", "nat", "bat",
        ]));
        let expect: Vec<Vec<String>> = generate_expected(vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]);
        assert!(compare(actual, expect))
    }

    #[test]
    fn example_two() {
        let actual = Solution::group_anagrams(generate_strs(vec![""]));
        let expect: Vec<Vec<String>> = generate_expected(vec![vec![""]]);
        assert!(compare(actual, expect))
    }

    #[test]
    fn example_three() {
        let actual = Solution::group_anagrams(generate_strs(vec!["a"]));
        let expect: Vec<Vec<String>> = generate_expected(vec![vec!["a"]]);
        assert!(compare(actual, expect))
    }
}
