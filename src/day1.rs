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
pub fn count_increases(depths: &[u32]) -> u32 {
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

#[aoc(day1, part2)]
pub fn depth_sliding_window(depths: &[u32]) -> u32 {
  let mut window: [u32; 3] = [0; 3];
  let mut w_cur: usize = 0;
  let mut sums: Vec<u32> = Vec::new();

  for (idx, depth) in depths[..].iter().enumerate() {
    window[w_cur] = *depth;
    // Keep cursor going around on window array
    w_cur = (w_cur + 1) % 3;
    if idx > 2 {
      sums.push(window.iter().sum());
    }
  }
  count_increases(&sums)
}
