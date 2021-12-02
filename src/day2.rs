// use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Directions {
  Forward,
  Down,
  Up,
}

// impl fmt::Display for Directions {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//       Directions::Forward => write!(f, "Forward"),
//       Directions::Down => write!(f, "Down"),
//       Directions::Up => write!(f, "Up"),
//     }
//   }
// }

#[derive(Debug, Copy, Clone)]
pub struct Command {
  direction: Directions,
  distance: u32,
}

#[aoc_generator(day2)]
pub fn command_stream(input: &str) -> Vec<Command> {
  input
    .lines()
    .map(|l| {
      let mut words = l.trim().splitn(2, ' ');
      let dir_word = words.next().unwrap();
      Command {
        direction: match dir_word {
          "forward" => Directions::Forward,
          "down" => Directions::Down,
          "up" => Directions::Up,
          _ => panic!("Wrong direction: {}", dir_word),
        },
        distance: words.next().unwrap().parse::<u32>().unwrap(),
      }
    })
    .inspect(|c| println!("{:?}", c))
    .collect()
}

#[aoc(day2, part1)]
pub fn calc_mul(commands: &Vec<Command>) -> i32 {
  // Separate horizontal and vertical commands
  let (hor, ver): (Vec<Command>, Vec<Command>) = commands
    .iter()
    .partition(|&c| matches!(c.direction, Directions::Forward));
  // Calculate total distances
  let hor: i32 = hor.iter().fold(0, |acc, c| acc + c.distance as i32);
  let ver: i32 = ver.iter().fold(0, |acc, c| match c.direction {
    Directions::Up => acc - c.distance as i32,
    Directions::Down => acc + c.distance as i32,
    _ => panic!("Wrong direction: {:?}", c.direction),
  });
  hor * ver
}

#[aoc(day2, part2)]
pub fn calc_mul_aim(commands: &Vec<Command>) -> u32 {
  let mut x = 0;
  let mut d = 0;
  let mut aim = 0;
  for c in commands.iter() {
    match c.direction {
      Directions::Forward => {
        x += c.distance;
        d += c.distance * aim;
      }
      Directions::Up => aim -= c.distance,
      Directions::Down => aim += c.distance,
    }
  }
  x * d
}
