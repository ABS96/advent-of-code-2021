use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn sub_positions(input: &str) -> Vec<u32> {
  input.split(",").map(|pos| pos.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn fuel_cost(positions: &Vec<u32>) -> u32 {
  positions
    .iter()
    .fold(
      (HashMap::new(), u32::MAX),
      |(mut costs, lowest_cost), pos| {
        let mut lowest = lowest_cost;
        if !costs.contains_key(pos) {
          let total_dist = positions
            .iter()
            .fold(0, |t, p| t + (u32::max(*pos, *p) - u32::min(*pos, *p)));
          costs.insert(*pos, total_dist);
          lowest = u32::min(lowest_cost, total_dist);
        }
        (costs, lowest)
      },
    )
    .1
}
