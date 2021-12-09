type HeightMap = Vec<Vec<u32>>;
struct Location {
  x: i32,
  y: i32,
}

#[aoc_generator(day9)]
pub fn parse_map(input: &str) -> HeightMap {
  input
    .lines()
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|height| height.to_digit(10).unwrap())
        .collect()
    })
    .collect()
}

fn neighbours(heights: &HeightMap, loc: &Location) -> Vec<u32> {
  [
    Location {
      x: loc.x + 1,
      y: loc.y,
    },
    Location {
      x: loc.x,
      y: loc.y - 1,
    },
    Location {
      x: loc.x - 1,
      y: loc.y,
    },
    Location {
      x: loc.x,
      y: loc.y + 1,
    },
  ]
  .iter()
  .filter(|n| n.x >= 0 && n.x < heights[0].len() as i32 && n.y >= 0 && n.y < heights.len() as i32)
  .map(|n| heights[n.y as usize][n.x as usize])
  .collect()
}

#[aoc(day9, part1)]
pub fn sum_risk_levels(heights: &HeightMap) -> u32 {
  heights
    .iter()
    .enumerate()
    .map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter(|(x, height)| {
          neighbours(
            heights,
            &Location {
              x: *x as i32,
              y: y as i32,
            },
          )
          .iter()
          .all(|n| n > height)
        })
        .map(|(_, height)| 1 + height)
        .sum::<u32>()
    })
    .sum()
}
