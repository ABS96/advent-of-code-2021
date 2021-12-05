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

fn map_field(vents: &Vents) -> Field {
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
    .filter(|v| v.a.x == v.b.x || v.a.y == v.b.y)
    .for_each(|v| {
      for x in cmp::min(v.a.x, v.b.x)..=cmp::max(v.a.x, v.b.x) {
        for y in cmp::min(v.a.y, v.b.y)..=cmp::max(v.a.y, v.b.y) {
          field[x][y] += 1;
        }
      }
    });
  field
}

#[aoc(day5, part1)]
pub fn count_overlaps(vents: &Vents) -> u32 {
  let field = map_field(vents);
  field
    .iter()
    .flatten()
    .fold(0, |acc, o| acc + if o >= &2 { 1 } else { 0 })
}
