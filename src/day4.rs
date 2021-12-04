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

fn get_score(rows: &Board, numbers: &[u32]) -> u32 {
  // If there's at least one row with all numbers marked
  if rows.iter().any(|row| {
    row
      .iter()
      .all(|num| numbers.iter().any(|marked| marked == num))
  }) {
    // Then return all numbers that haven't yet been marked
    rows
      .iter()
      .flatten()
      .filter(|num| numbers.iter().all(|marked| &marked != num))
      .sum()
  } else {
    0
  } // Otherwise return 0
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

#[aoc(day4, part1)]
pub fn bingo_score(bingo: &Bingo) -> u32 {
  // 5 numbers must be drawn before a win can occur, so start from there
  // n is the number of items we will consider
  for n in 5..=bingo.numbers.len() {
    // Let's be careful not to overindex the drawn numbers here!
    let drawn = &bingo.numbers[0..n];
    let highest_score = bingo
      .boards
      .iter()
      .map(|board| {
        let row_score = get_score(&board, &drawn);
        let column_score = get_score(&transpose_board(&board), &drawn);
        cmp::max(row_score, column_score)
      })
      .max()
      .unwrap(); // iterator::max() yields an Option, but here it will always be Some
    if highest_score > 0 {
      let winning_number = *drawn.iter().last().unwrap();
      println!(
        "In round {}, {} will be called, and a board will win with a score of {}",
        n, winning_number, highest_score
      );
      return highest_score * winning_number;
    }
  }
  println!("Nobody wins.");
  return 0;
}
