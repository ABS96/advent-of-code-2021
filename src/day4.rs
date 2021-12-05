use std::cmp;

type Row = Vec<u32>;
type Board = Vec<Row>;

#[derive(Debug)]
pub struct Bingo {
  numbers: Vec<u32>,
  boards: Vec<Board>,
}

// 1 line with the drawn numbers (comma-separated),
// then an arbitrary number of boards:
//   1 empty line + 5 lines of board data:
//   - Each of these lines represents a row
//   - Numbers on a row are separated by whitespace
#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Bingo {
  let mut lines = input.lines();
  Bingo {
    numbers: lines
      .next()
      .unwrap()
      .trim()
      .split(',')
      .map(|n| n.parse().unwrap())
      .collect(),
    boards: lines
      .collect::<Vec<_>>() // Type must be instantiated for chunks() to work
      .chunks(6) // Group remaining lines into batches of 6
      .map(|l| {
        l[1..] // Skip empty line
          .iter()
          .map(|r| {
            r.trim()
              .split_whitespace()
              .take(5) // Limit to 5 numbers
              .map(|n| n.parse().unwrap())
              .collect()
          })
          .collect()
      })
      .collect(),
  }
}

fn get_score(rows: &Board, numbers: &[u32]) -> Option<u32> {
  // If there's at least one row with all numbers marked,
  if rows.iter().any(|row| {
    row
      .iter()
      .all(|num| numbers.iter().any(|marked| marked == num))
  }) {
    // then return the sum of all unmarked numbers
    Some(
      rows
        .iter()
        .flatten()
        .filter(|num| numbers.iter().all(|marked| &marked != num))
        .sum(),
    )
  } else {
    None
  }
}

fn transpose_board(board: &Board) -> Board {
  board[0]
    .iter()
    .enumerate()
    .map(|(i_col, _)| {
      // Take each column's index
      board
        .iter()
        .enumerate()
        .map(|(i_row, _)| {
          // Put each element from an original column into a new row
          board[i_row][i_col]
        })
        .collect()
    })
    .collect()
}

fn evaluate_boards(bingo: &Bingo, drawn: &[u32]) -> Vec<Option<u32>> {
  // Let's be careful not to overindex the drawn numbers here!
  bingo
    .boards
    .iter()
    .map(|board| {
      let row_score = get_score(&board, &drawn);
      let column_score = get_score(&transpose_board(&board), &drawn);
      cmp::max(row_score, column_score) // This can handle Options, awesome!
    })
    .collect()
}

#[aoc(day4, part1)]
pub fn bingo_score(bingo: &Bingo) -> u32 {
  // 5 numbers must be drawn before a win can occur, so start from there
  for round in 5..=bingo.numbers.len() {
    let drawn = &bingo.numbers[0..round];
    let scores = evaluate_boards(bingo, drawn);
    let first_score = scores.iter().find(|score| score.is_some());
    if first_score.is_some() {
      println!("First win occurs at round {}", round);
      return first_score.unwrap().unwrap() * drawn.last().unwrap();
    }
  }
  return 0;
}

#[aoc(day4, part2)]
pub fn bingo_last_score(bingo: &Bingo) -> u32 {
  let mut idx_incomplete = 0;
  for round in 5..=bingo.numbers.len() {
    let drawn = &bingo.numbers[0..round];
    let scores = evaluate_boards(bingo, drawn);
    if scores.iter().all(|score| score.is_some()) {
      println!("Last win occurs at round {}", round);
      return scores[idx_incomplete].unwrap() * drawn.last().unwrap();
    } else {
      idx_incomplete = scores.iter().position(|score| score.is_none()).unwrap();
    }
  }
  return 0;
}
