use regex::Regex;
use std::collections::HashSet;

#[derive(PartialEq)]
enum FoldDirection {
  AlongX,
  AlongY,
}

pub struct FoldLine {
  direction: FoldDirection,
  location: i32,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Point {
  x: i32,
  y: i32,
}

type Paper = HashSet<Point>;

pub trait Fold {
  fn fold(&mut self, line: &FoldLine);
}

pub trait Print {
  fn print(&self) -> String;
}

impl Fold for Paper {
  fn fold(&mut self, line: &FoldLine) {
    let mut paper_new: HashSet<Point> = HashSet::new();
    for dot in self.iter() {
      let dot_new = Point {
        x: if line.direction == FoldDirection::AlongX && dot.x > line.location {
          line.location - line.location.abs_diff(dot.x) as i32
        } else {
          dot.x
        },
        y: if line.direction == FoldDirection::AlongY && dot.y > line.location {
          line.location - line.location.abs_diff(dot.y) as i32
        } else {
          dot.y
        },
      };
      paper_new.insert(dot_new);
    }
    *self = paper_new;
  }
}

impl Print for Paper {
  fn print(&self) -> String {
    let x_max = self.iter().map(|dot| dot.x).max().unwrap() + 1;
    let y_max = self.iter().map(|dot| dot.y).max().unwrap() + 1;
    let mut result = String::with_capacity(((x_max + 1) * y_max) as usize);
    for y in 0..y_max {
      result.push('\n');
      for x in 0..x_max {
        result.push(if self.contains(&Point { x, y }) {
          '#'
        } else {
          ' '
        });
      }
    }
    result
  }
}

#[aoc_generator(day13)]
pub fn parse_paper(input: &str) -> (Paper, Vec<FoldLine>) {
  let re_dot = Regex::new(r"(\d+),(\d+)").unwrap();
  let re_fold = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
  (
    re_dot
      .captures_iter(input)
      .map(|dot| Point {
        x: dot[1].parse().unwrap(),
        y: dot[2].parse().unwrap(),
      })
      .collect(),
    re_fold
      .captures_iter(input)
      .map(|fold| FoldLine {
        direction: match &fold[1] {
          "x" => Some(FoldDirection::AlongX),
          "y" => Some(FoldDirection::AlongY),
          _ => None,
        }
        .expect("err"),
        location: fold[2].parse().unwrap(),
      })
      .collect(),
  )
}

#[aoc(day13, part1)]
pub fn count_dots_first_iter((dots, folds): &(Paper, Vec<FoldLine>)) -> usize {
  let mut paper = dots.clone();
  paper.fold(&folds[0]);
  paper.iter().count()
}

#[aoc(day13, part2)]
pub fn show_code((dots, folds): &(Paper, Vec<FoldLine>)) -> String {
  let mut paper = dots.clone();
  for fold in folds {
    paper.fold(&fold);
  }
  paper.print()
}
