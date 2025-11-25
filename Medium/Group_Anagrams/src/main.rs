struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for s in strs {
            let key = Self::normalize_for_anagram(&s);
            anagram_map.entry(key).or_default().push(s);
        }

        anagram_map.into_values().collect()
    }

    fn normalize_for_anagram(s: &str) -> String {
        let mut chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        chars.sort_unstable();
        chars.into_iter().collect()
    }
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let result = Solution::group_anagrams(strs);
    println!("{:?}", result);
}
