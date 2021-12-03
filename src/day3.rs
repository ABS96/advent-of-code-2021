const DIAG_BITS: usize = 12;

// Each line contains a number in binary notation.
#[aoc_generator(day3)]
pub fn get_diagnostic(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|l| u32::from_str_radix(l.trim(), 2).unwrap())
    .collect()
}

// The submarine's power is determined by γ * ε.
// γ is composed of the most frequent digits in each binary position.
// ε is composed of the least frequent digits.
#[aoc(day3, part1)]
pub fn get_sub_power(diag: &Vec<u32>) -> u32 {
  // It is sufficient to find γ, as ε can be obtained by negating γ.
  // 'weights' will keep track of how many times a '1' is encountered.
  // The first element is the LSB.
  let mut weights: [i32; DIAG_BITS] = [0; DIAG_BITS];
  diag.iter().for_each(|data| {
    for pow in 0..DIAG_BITS {
      weights[pow] += match (data >> pow) & 1 {
        1 => 1,
        0 => -1,
        _ => panic!("This program cannot run on a quantum computer!"),
      };
    }
    // println!("{:0width$b} -> {:?}", data, weights, width = DIAG_BITS);
  });
  // Edge case: if an equal number of 0s and 1s were encountered,
  // the digit would be 0.
  let gamma: u32 = weights.iter().enumerate().fold(0, |gam, (pow, weight)| {
    let digit = if weight > &0 { 1 } else { 0 };
    let digit = digit << pow;
    // println!("{:0width$b} | {:0width$b}", gam, digit, width = DIAG_BITS);
    gam | digit
  });
  let epsilon: u32 = !gamma & ((1 << DIAG_BITS) - 1);
  println!(
    "γ = {}, ε = {}, power = {}",
    gamma,
    epsilon,
    gamma * epsilon
  );
  gamma * epsilon
}
