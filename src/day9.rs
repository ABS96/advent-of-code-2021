use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Position {
  x: i32,
  y: i32,
}
impl PartialEq for Position {
  fn eq(&self, other: &Position) -> bool {
    self.x == other.x && self.y == other.y
  }
}

#[derive(Clone, Copy)]
struct Height {
  pos: Position,
  val: u32,
}
impl PartialEq for Height {
  fn eq(&self, other: &Height) -> bool {
    self.pos == other.pos && self.val == other.val
  }
}

type HeightMap = Vec<Vec<u32>>;
trait Neighbours {
  fn neighbours(&self, node: &Position) -> Vec<Height>;
}
trait FindLowPoints {
  fn find_low_points(&self) -> Vec<Height>;
}
trait ExploreBasin {
  fn explore_basin(&self, bottom: &Height) -> Vec<Height>;
}

impl Neighbours for HeightMap {
  fn neighbours(&self, pos: &Position) -> Vec<Height> {
    [(1, 0), (0, -1), (-1, 0), (0, 1)]
      .iter()
      .map(|(x, y)| Position {
        x: pos.x + x,
        y: pos.y + y,
      })
      .filter(|n| {
        (0..self[0].len()).contains(&(n.x as usize)) && (0..self.len()).contains(&(n.y as usize))
      })
      .map(|n| Height {
        pos: n,
        val: self[n.y as usize][n.x as usize],
      })
      .collect()
  }
}

impl FindLowPoints for HeightMap {
  fn find_low_points(&self) -> Vec<Height> {
    self
      .iter()
      .enumerate()
      .map(|(y, row)| {
        row
          .iter()
          .enumerate()
          .filter(move |(x, height)| {
            self
              .neighbours(&Position {
                x: *x as i32,
                y: y as i32,
              })
              .iter()
              .all(|h| h.val > **height)
          })
          .map(move |(x, height)| Height {
            pos: Position {
              x: x as i32,
              y: y as i32,
            },
            val: *height,
          })
      })
      .flatten()
      .collect()
  }
}

impl ExploreBasin for HeightMap {
  fn explore_basin(&self, bottom: &Height) -> Vec<Height> {
    let mut basin: Vec<Height> = Vec::new();
    let mut queue = VecDeque::from([*bottom]);
    while !queue.is_empty() {
      let node = queue.pop_front().unwrap();
      if !basin.contains(&node) && node.val < 9 {
        basin.push(node.clone());
        self
          .neighbours(&node.pos)
          .iter()
          .for_each(|neighbour| queue.push_back(*neighbour));
      }
    }
    basin
  }
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

#[aoc(day9, part1)]
pub fn sum_risk_levels(heights: &HeightMap) -> u32 {
  heights
    .find_low_points()
    .iter()
    .map(|height| 1 + height.val)
    .sum()
}

#[aoc(day9, part2)]
pub fn largest_basins(heights: &HeightMap) -> usize {
  let mut basin_sizes: Vec<usize> = heights
    .find_low_points()
    .iter()
    .map(|point| heights.explore_basin(&point).len())
    .collect();
  basin_sizes.sort();
  basin_sizes.iter().rev().take(3).product()
}
