use maplit::hashmap;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn solve(
    text: &String
  ) -> u32 {
    let balloon: HashMap<char, u32> = hashmap!{ 'b' => 1, 'a' => 1, 'l' => 2, 'o' => 2, 'n' => 1 };

    let chars: Vec<char> = (*text).chars().collect();
    let len = chars.len();

    let mut occurrences: HashMap<char, u32> = hashmap!{};
    chars.iter()
      .for_each(|ch| *occurrences.entry(*ch).or_insert(0) += 1);

    balloon.iter()
      .map(|(ch, counter)| *occurrences.entry(*ch).or_insert(0) / *counter)
      .fold(len as u32, |prev, count| std::cmp::min(prev, count))
  }
}