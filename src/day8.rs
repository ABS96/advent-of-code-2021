use std::collections::HashMap;

// A 7-bit flag which represents which segments are on
// LSB (2⁰) signals whether 'a' is on
// MSB (2⁶) signals whether 'g' is on
type SignalPattern = u8;
const ALL_SEGMENTS: SignalPattern = 0b1111111;

pub struct BadDisplay {
  patterns: [SignalPattern; 10],
  value: [SignalPattern; 4],
}

fn parse_pattern(input: &str) -> SignalPattern {
  input
    .chars()
    .fold(0, |out, c| out | (1 << (c as u8 - 'a' as u8)))
}

#[aoc_generator(day8)]
pub fn process_lines(input: &str) -> Vec<BadDisplay> {
  input
    .lines()
    .map(|line| {
      let line_parts: Vec<Vec<SignalPattern>> = line
        .split('|')
        .map(|pattern| {
          pattern
            .trim()
            .split_whitespace()
            .map(|s| parse_pattern(s))
            .collect()
        })
        .collect();
      BadDisplay {
        patterns: line_parts[0].clone().try_into().unwrap(),
        value: line_parts[1].clone().try_into().unwrap(),
      }
    })
    .collect()
}

fn binary_weight(number: &u8) -> u8 {
  (0..u8::BITS).fold(0, |w, i| w + ((number >> i) & 1))
}

#[aoc(day8, part1)]
pub fn count_easy_digits(entries: &Vec<BadDisplay>) -> usize {
  entries
    .iter()
    .map(|entry| {
      entry
        .value
        .iter()
        .filter(|digit| {
          let weight = binary_weight(digit);
          [2u8, 4u8, 3u8, 7u8].contains(&weight)
        })
        .count()
    })
    .sum()
}

fn decode_value(entry: &BadDisplay) -> u32 {
  // Determine patterns for digits by unique binary weight
  let mut digits: HashMap<u8, SignalPattern> = HashMap::new();
  digits.insert(8, ALL_SEGMENTS);
  [(1, 2), (4, 4), (7, 3)]
    .iter()
    .for_each(|(number, weight)| {
      digits.insert(
        *number,
        *entry
          .patterns
          .iter()
          .find(|pattern| binary_weight(pattern) == *weight)
          .unwrap(),
      );
    });

  // Determine segments by their frequency of occurrence in patterns
  let mut segments: HashMap<char, SignalPattern> = HashMap::new();
  for i_seg in 0..7 {
    let freq = entry
      .patterns
      .iter()
      .filter(|pat| 1 & (*pat >> i_seg) == 1)
      .count();
    match freq {
      4 => segments.insert('e', 1 << i_seg),
      6 => segments.insert('b', 1 << i_seg),
      9 => segments.insert('f', 1 << i_seg),
      _ => None,
    };
  }

  // Determine the rest of the segments
  let b_and_d_on = digits[&4] & !digits[&1]; // Segments b, d
  let middle_column_on = entry // Segments a, d, g
    .patterns
    .iter()
    .filter(|p| binary_weight(p) == 5)
    .fold(ALL_SEGMENTS, |u, p| u & p);
  segments.insert('a', digits[&7] & !digits[&1]);
  segments.insert('d', b_and_d_on & middle_column_on);
  segments.insert('g', middle_column_on & !(segments[&'a'] | segments[&'d']));
  segments.insert('c', segments.values().fold(ALL_SEGMENTS, |c, s| c & !s));

  // Determine patterns for the rest of the digits
  [
    (0, "abcefg"),
    (2, "acdeg"),
    (3, "acdfg"),
    (5, "abdfg"),
    (6, "abdefg"),
    (9, "abcdfg"),
  ]
  .iter()
  .for_each(|(num, segs)| {
    let pattern = segs.chars().fold(0u8, |pat, c| pat | segments[&c]);
    digits.insert(*num, pattern);
  });

  // Pattern-match each digit, then reconstruct value
  entry
    .value
    .iter()
    .map(|d| {
      digits
        .iter()
        .find_map(|(num, pat)| if d == pat { Some(*num as u32) } else { None })
        .unwrap()
    })
    .rev()
    .enumerate()
    .fold(0, |value, (power_of_10, digit)| {
      value + digit * &10u32.pow(power_of_10 as u32)
    })
}

#[aoc(day8, part2)]
pub fn decoded_sum(entries: &Vec<BadDisplay>) -> u32 {
  entries.iter().map(|entry| decode_value(entry)).sum()
}
