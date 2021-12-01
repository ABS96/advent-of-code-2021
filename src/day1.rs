use std::cmp::Ordering;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|l| {
      let depth = l.trim().parse().unwrap();
      depth
    })
    .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(depths: &[u32]) -> u32 {
  let mut last_depth: Option<u32> = None;
  let mut increases: u32 = 0;

  for depth in depths.iter() {
    print!("{} ", *depth);
    match last_depth {
      None => print!("N/A - no previous measurement"),
      Some(last) => match depth.cmp(&last) {
        Ordering::Less => print!("(decreased)"),
        Ordering::Equal => print!("(unchanged)"),
        Ordering::Greater => {
          print!("(increased)");
          increases += 1;
        }
      },
    }
    print!("\n");
    last_depth = Some(*depth);
  }

  increases
}
