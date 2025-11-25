struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        Self::normalize_for_anagram(&s) == Self::normalize_for_anagram(&t)
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
    let s = String::from("anagram");
    let t = String::from("nagaram");
    let result = Solution::is_anagram(s, t);
    println!("{}", result);
}
