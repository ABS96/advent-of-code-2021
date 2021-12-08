type SignalPattern = u8;

#[derive(Debug)]
pub struct BadDisplay {
  patterns: [SignalPattern; 10],
  digits: [SignalPattern; 4],
}

// Converts a pattern into a 7-bit flag
// LSB (2⁰) signals whether 'a' is present
// MSB (2⁶) signals whether 'g' is present
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
      let l: Vec<Vec<SignalPattern>> = line
        .split('|')
        .map(|p| {
          p.trim()
            .split_whitespace()
            .map(|s| parse_pattern(s))
            .collect()
        })
        .collect();
      BadDisplay {
        patterns: l[0].clone().try_into().unwrap(),
        digits: l[1].clone().try_into().unwrap(),
      }
    })
    .collect()
}

fn weight(number: &u8) -> u8 {
  (0..u8::BITS).fold(0, |w, i| w + ((number >> i) & 1))
}

#[aoc(day8, part1)]
pub fn count_easy_digits(entries: &Vec<BadDisplay>) -> usize {
  entries
    .iter()
    .map(|e| {
      e.digits
        .iter()
        .filter(|digit| [2u8, 4u8, 3u8, 7u8].contains(&weight(digit)))
        .count()
    })
    .sum()
}
