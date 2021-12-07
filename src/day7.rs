#[derive(Copy, Clone, Debug)]
enum FuelBurnRate {
  Constant,
  Increasing,
}

#[aoc_generator(day7)]
pub fn subs(input: &str) -> Vec<u32> {
  input.split(",").map(|pos| pos.parse().unwrap()).collect()
}

fn fuel_cost(x1: u32, x2: u32, rate: FuelBurnRate) -> u32 {
  let dist = u32::max(x1, x2) - u32::min(x1, x2);
  match rate {
    FuelBurnRate::Constant => dist,
    FuelBurnRate::Increasing => (dist * (dist + 1)) / 2, // triangular number
  }
}

fn lowest_cost(subs: &Vec<u32>, rate: FuelBurnRate) -> u32 {
  // consider all positions from the leftmost to the rightmost submarine
  (0..=*subs.iter().max().unwrap()).fold(u32::MAX, |lowest, pos| {
    u32::min(
      lowest,
      subs
        // for each position, calculate the cost of all submarines going there
        .iter()
        .fold(0, |total, sub| total + fuel_cost(pos as u32, *sub, rate)),
    )
  })
}

#[aoc(day7, part1)]
pub fn part1(positions: &Vec<u32>) -> u32 {
  lowest_cost(positions, FuelBurnRate::Constant)
}

#[aoc(day7, part2)]
pub fn part2(positions: &Vec<u32>) -> u32 {
  lowest_cost(positions, FuelBurnRate::Increasing)
}
