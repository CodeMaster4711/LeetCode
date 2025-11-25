use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let key = Self::get_char_count(&s);
            anagram_map.entry(key).or_default().push(s);
        }
        anagram_map.into_values().collect()
    }

    fn get_char_count(s: &str) -> [u32; 26] {
        let mut count = [0u32; 26];
        for c in s.chars() {
            if c.is_ascii_lowercase() {
                let idx = (c as u8 - b'a') as usize;
                count[idx] += 1;
            }
        }
        count
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
