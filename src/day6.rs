#[aoc_generator(day6)]
pub fn parse_fish_ages(input: &str) -> Vec<u8> {
  input.split(',').fold(Vec::new(), |mut ages, a| {
    ages.push(a.parse::<u8>().unwrap());
    ages
  })
}

#[aoc(day6, part1)]
pub fn population_80(initial_ages: &Vec<u8>) -> usize {
  let mut ages = initial_ages.clone();
  for _ in 0..80 {
    let new_fish: usize = ages.iter_mut().fold(0, |new_fish, age| {
      if age == &0 {
        *age = 6;
        new_fish + 1
      } else {
        *age -= 1;
        new_fish
      }
    });
    ages.append(&mut vec![8; new_fish]);
  }
  ages.len()
}
