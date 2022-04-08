use num;
use std::collections::HashSet;

const GRID_SIZE: usize = 10;
type OctoGrid = [[u32; GRID_SIZE]; GRID_SIZE];

#[derive(Clone, Copy, Hash)]
struct OctoPos {
  x: usize,
  y: usize,
}
impl PartialEq for OctoPos {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl Eq for OctoPos {}

#[aoc_generator(day11)]
pub fn parse_octogrid(input: &str) -> OctoGrid {
  input
    .lines()
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
    })
    .collect::<Vec<[u32; GRID_SIZE]>>()
    .try_into()
    .unwrap()
}

fn neighbours(pos: OctoPos) -> Vec<OctoPos> {
  let mut ns: Vec<OctoPos> = Vec::new();
  let y_min = num::clamp(pos.y as i32 - 1, 0, GRID_SIZE as i32 - 1) as usize;
  let x_min = num::clamp(pos.x as i32 - 1, 0, GRID_SIZE as i32 - 1) as usize;
  let y_max = num::clamp(pos.y as i32 + 1, 0, GRID_SIZE as i32 - 1) as usize;
  let x_max = num::clamp(pos.x as i32 + 1, 0, GRID_SIZE as i32 - 1) as usize;
  for y in y_min..=y_max {
    for x in x_min..=x_max {
      if !(x == pos.x && y == pos.y) {
        ns.push(OctoPos { x, y });
      }
    }
  }
  return ns;
}

fn step_grid(grid: &mut OctoGrid) -> u32 {
  let mut flashed = HashSet::new();
  let mut times = 0;
  for row in grid.iter_mut() {
    for octopus in row {
      *octopus += 1;
    }
  }
  loop {
    let mut new_flash = 0;
    for y in 0..GRID_SIZE {
      for x in 0..GRID_SIZE {
        let position = OctoPos { x, y };
        if !flashed.contains(&position) && grid[y][x] >= 10 {
          for n in neighbours(position) {
            grid[n.y][n.x] += 1;
          }
          flashed.insert(position);
          new_flash += 1;
        }
      }
    }
    if new_flash < 1 {
      break;
    }
  }
  for o in flashed {
    grid[o.y][o.x] = 0;
    times += 1;
  }
  times
}

#[aoc(day11, part1)]
pub fn total_flashes_100(input: &OctoGrid) -> u32 {
  let mut grid = input.clone();
  let mut flashes = 0;
  for _ in 0..100 {
    flashes += step_grid(&mut grid);
  }
  flashes
}

#[aoc(day11, part2)]
pub fn first_simultaneous_flash(input: &OctoGrid) -> u32 {
  let mut grid = input.clone();
  let mut flashes = 0;
  let mut steps = 0;
  while flashes != num::pow(GRID_SIZE as u32, 2) {
    steps += 1;
    flashes = step_grid(&mut grid);
  }
  steps
}
