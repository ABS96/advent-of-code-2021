use std::cmp::Ordering;

const DIAG_BITS: usize = 12;

// Each line contains a number in binary notation.
#[aoc_generator(day3)]
pub fn get_diagnostic(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|l| u32::from_str_radix(l.trim(), 2).unwrap())
    .collect()
  // vec![
  //   0b00100, 0b11110, 0b10110, 0b10111, 0b010101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
  //   0b00010, 0b01010,
  // ]
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
  // the digit would,a be 0.
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

enum BitCriterion {
  OxygenGeneratorRating,
  Co2ScrubberRating,
}

fn find_by_bit_criterion(diag: &Vec<u32>, crit: BitCriterion) -> u32 {
  let mut candidates = diag.clone();
  for digit in (0..DIAG_BITS).rev() {
    // println!("Candidates: {:?}", candidates);
    // println!("DIGIT {}", digit + 1);
    let (prt_1, prt_0): (Vec<u32>, Vec<u32>) =
      candidates.iter().partition(|d| (*d >> digit) & 1 == 1);
    // println!("0: {:?}", prt_0);
    // println!("1: {:?}", prt_1);
    candidates = match crit {
      BitCriterion::OxygenGeneratorRating => match prt_0.len().cmp(&prt_1.len()) {
        Ordering::Greater => prt_0, // Keep 0s if there are more of them
        _ => prt_1,                 // Otherwise, keeps 1s
      },
      BitCriterion::Co2ScrubberRating => match prt_1.len().cmp(&prt_0.len()) {
        Ordering::Less => prt_1, // Keep 1s if there are less of them
        _ => prt_0,              // Otherwise, keep 0s
      },
    }
    .clone();
    if candidates.len() == 1 {
      break;
    }
  }
  candidates[0]
}

#[aoc(day3, part2)]
pub fn get_life_support_rating(diag: &Vec<u32>) -> u32 {
  let oxygen_generator_rating = find_by_bit_criterion(diag, BitCriterion::OxygenGeneratorRating);
  let co2_scrubber_rating = find_by_bit_criterion(diag, BitCriterion::Co2ScrubberRating);
  let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
  println!(
    "oxygen: {}, CO₂: {}, life support: {}",
    oxygen_generator_rating, co2_scrubber_rating, life_support_rating
  );
  life_support_rating
}
