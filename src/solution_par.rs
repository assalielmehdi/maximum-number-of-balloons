use maplit::hashmap;
use std::collections::HashMap;
use rayon::join;

pub struct Solution;

impl Solution {
  fn count_occurrences_single(
    chars: Vec<char>,
    occurrences: &mut HashMap<char, u32>,
    balloon: &HashMap<char, u32>
  ) {
    chars.iter().for_each(|ch| {
      if (*balloon).contains_key(ch) {
        *(*occurrences).entry(*ch).or_insert(0) += 1
      }
    });
  }

  fn merge_occurrences(
    mut left_occurrences: HashMap<char, u32>,
    mut right_occurrences: HashMap<char, u32>,
    merged_occurrences: &mut HashMap<char, u32>,
    balloon: &HashMap<char, u32>
  ) {
    (*balloon).iter()
      .for_each(|(ch, _)| {
        (*merged_occurrences).insert(
          *ch,
          *left_occurrences.entry(*ch).or_insert(0) + *right_occurrences.entry(*ch).or_insert(0)
        ); 
      });
  }

  fn count_occurrences(
    chars: Vec<char>,
    occurrences: &mut HashMap<char, u32>,
    balloon: &HashMap<char, u32>,
    level: u32
  ) {
    if level == 0 {
      Solution::count_occurrences_single(chars, occurrences, balloon);
    } else {
      let (left, right) = chars.split_at(chars.len() / 2);
      let (mut left_occurrences, mut right_occurrences) = (hashmap!{}, hashmap!{});
      join(
        || Solution::count_occurrences(left.to_vec(), &mut left_occurrences, balloon, level - 1),
        || Solution::count_occurrences(right.to_vec(), &mut right_occurrences, balloon, level - 1)
      );
      Solution::merge_occurrences(left_occurrences, right_occurrences, occurrences, balloon);
    }
  }

  pub fn solve(
    text: &String,
    levels: u32
  ) -> u32 {
    let balloon: HashMap<char, u32> = hashmap!{ 'b' => 1, 'a' => 1, 'l' => 2, 'o' => 2, 'n' => 1 };

    let chars: Vec<char> = (*text).chars().collect();
    let len = chars.len();

    let mut occurrences = hashmap!{};
    Solution::count_occurrences(chars, &mut occurrences, &balloon, levels);

    balloon.iter()
      .map(|(ch, counter)| *occurrences.entry(*ch).or_insert(0) / *counter)
      .fold(len as u32, |prev, count| std::cmp::min(prev, count))
  }
}