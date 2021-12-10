#[aoc_generator(day10)]
pub fn read_lines(input: &str) -> Vec<String> {
  input
    .lines()
    .map(|line| String::from(line.trim()))
    .collect()
}

fn get_pair(c: &char) -> Option<char> {
  match c {
    ')' => Some('('),
    ']' => Some('['),
    '}' => Some('{'),
    '>' => Some('<'),
    _ => None,
  }
}

fn corruption_score(c: &char) -> u32 {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => 0,
  }
}

fn completion_score(c: &char) -> u32 {
  match c {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4,
    _ => 0,
  }
}

enum LineStatus {
  Incomplete(Vec<char>),
  Corrupted(char),
  ClosedEarly,
}

fn process_lines(lines: &Vec<String>) -> Vec<LineStatus> {
  lines
    .iter()
    .map(|line| {
      let mut stack: Vec<char> = Vec::new();
      let mut status = LineStatus::Incomplete(Vec::new());
      for c in line.chars() {
        match get_pair(&c) {
          Some(closer_pair) => match stack.last() {
            None => {
              status = LineStatus::ClosedEarly;
              break;
            }
            Some(opener) => {
              if closer_pair == *opener {
                stack.pop();
                continue;
              } else {
                status = LineStatus::Corrupted(c);
                break;
              }
            }
          },
          None => {
            stack.push(c);
          }
        }
      }
      if matches!(status, LineStatus::Incomplete(_)) {
        status = LineStatus::Incomplete(stack)
      }
      status
    })
    .collect()
}

#[aoc(day10, part1)]
pub fn error_score(lines: &Vec<String>) -> u32 {
  process_lines(lines)
    .iter()
    .map(|status| match status {
      LineStatus::Corrupted(by) => corruption_score(&by),
      _ => 0,
    })
    .sum()
}

#[aoc(day10, part2)]
pub fn middle_score(lines: &Vec<String>) -> u64 {
  let mut scores = process_lines(lines)
    .iter()
    .filter_map(|status| match status {
      LineStatus::Incomplete(stack) => Some(
        stack
          .iter()
          .rev()
          .fold(0u64, |score, c| score * 5 + completion_score(&c) as u64),
      ),
      _ => None,
    })
    .collect::<Vec<u64>>();
  scores.sort_unstable();
  scores[scores.len() / 2]
}
