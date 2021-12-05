use std::cmp;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
  x: usize,
  y: usize,
}

#[derive(Debug)]
pub struct Vent {
  a: Coordinate,
  b: Coordinate,
}

type Vents = Vec<Vent>;
type Field = Vec<Vec<u32>>;

enum Directions {
  Straight,
  Mixed,
}

#[aoc_generator(day5)]
pub fn parse_field(input: &str) -> Vents {
  input
    .lines()
    .map(|line| {
      let endpoints: Vec<Coordinate> = line
        .split(" -> ")
        .map(|location| {
          let coords: Vec<usize> = location
            .split(",")
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
          Coordinate {
            x: coords[0],
            y: coords[1],
          }
        })
        .collect();
      Vent {
        a: endpoints[0],
        b: endpoints[1],
      }
    })
    .collect()
}

fn map_field(vents: &Vents, directions: Directions) -> Field {
  let (xs, ys): (Vec<usize>, Vec<usize>) = vents
    .iter()
    .map(|vent| (cmp::max(vent.a.x, vent.b.x), cmp::max(vent.a.y, vent.b.y)))
    .unzip();
  let size_x = xs.iter().max().unwrap() + 1;
  let size_y = ys.iter().max().unwrap() + 1;
  println!("Field is {}×{}", size_x, size_y);
  let mut field = vec![vec![0; size_y]; size_x]; // [x][y]
  vents
    .iter()
    .filter(|v| matches!(directions, Directions::Mixed) || v.a.x == v.b.x || v.a.y == v.b.y)
    .for_each(|v| {
      let dir_x = v.a.x.cmp(&v.b.x);
      let dir_y = v.a.y.cmp(&v.b.y);
      let length = if !matches!(dir_x, cmp::Ordering::Equal) {
        cmp::max(v.a.x, v.b.x) - cmp::min(v.a.x, v.b.x)
      } else {
        cmp::max(v.a.y, v.b.y) - cmp::min(v.a.y, v.b.y)
      };
      for i in 0..=length {
        let i_x = match dir_x {
          cmp::Ordering::Equal => v.a.x,
          cmp::Ordering::Greater => v.a.x - i,
          cmp::Ordering::Less => v.a.x + i,
        };
        let i_y = match dir_y {
          cmp::Ordering::Equal => v.a.y,
          cmp::Ordering::Greater => v.a.y - i,
          cmp::Ordering::Less => v.a.y + i,
        };
        field[i_x][i_y] += 1;
      }
    });
  // for y in 0..size_y {
  //   for x in 0..size_x {
  //     let p = field[x][y];
  //     if p == 0 {
  //       print!(".");
  //     } else {
  //       print!("{}", p);
  //     }
  //   }
  //   print!("\n");
  // }
  field
}

fn count_overlaps(field: &Field) -> u32 {
  field
    .iter()
    .flatten()
    .fold(0, |acc, o| acc + if o >= &2 { 1 } else { 0 })
}

#[aoc(day5, part1)]
pub fn count_straight_overlaps(vents: &Vents) -> u32 {
  count_overlaps(&map_field(vents, Directions::Straight))
}

#[aoc(day5, part2)]
pub fn count_all_overlaps(vents: &Vents) -> u32 {
  count_overlaps(&map_field(vents, Directions::Mixed))
}
