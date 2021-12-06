const YOUNG_CYCLE: usize = 8;
const MATURE_CYCLE: usize = 6;

#[aoc_generator(day6)]
pub fn parse_fish_ages(input: &str) -> Vec<usize> {
  input.split(',').fold(Vec::new(), |mut ages, a| {
    ages.push(a.parse::<usize>().unwrap());
    ages
  })
}

fn age_counts(ages: &Vec<usize>) -> Vec<u64> {
  ages
    .iter()
    .fold(vec![0; YOUNG_CYCLE + 1], |mut counts, age| {
      counts[YOUNG_CYCLE - *age] += 1; // first is youngest, last is spawning
      counts
    })
}

fn population(initial_ages: &Vec<usize>, days: usize) -> u64 {
  // Number of fish selected by day of their cycles
  (0..days)
    .into_iter()
    .fold(age_counts(initial_ages), |mut fish, _| {
      let spawning = fish.pop().unwrap(); // number of new fish
      let mut f = vec![spawning]; // construct new day
      f.extend(&fish); // grow existing fish
      f[YOUNG_CYCLE - MATURE_CYCLE] += spawning; // reset parents' cycle
      f
    })
    .iter()
    .sum()
}

#[aoc(day6, part1)]
pub fn population_80(initial_ages: &Vec<usize>) -> u64 {
  population(initial_ages, 80)
}

#[aoc(day6, part2)]
pub fn population_256(initial_ages: &Vec<usize>) -> u64 {
  population(initial_ages, 256)
}
