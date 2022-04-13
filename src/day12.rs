use std::collections::HashMap;
use std::collections::HashSet;

fn map_cave(input: &str) -> HashMap<&str, HashSet<&str>> {
  input
    .lines()
    .map(|line| {
      line
        .trim()
        .split('-')
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap()
    })
    .fold(HashMap::new(), |mut c, nodes: [&str; 2]| {
      for (i, node) in nodes.iter().enumerate() {
        match c.get_mut(node) {
          Some(n) => n.insert(nodes[1 - i]),
          None => {
            c.insert(node, HashSet::new());
            c.get_mut(node).unwrap().insert(nodes[1 - i]);
            false
          }
        };
      }
      c
    })
}

fn traverse(
  cave: &HashMap<&str, HashSet<&str>>,
  node: &str,
  path: Vec<String>,
  routes: &mut Vec<Vec<String>>,
  twice: Option<&str>,
) {
  let mut new_path = path.clone();
  new_path.push(node.to_string());
  if node == "end" {
    routes.push(new_path);
  } else if node.chars().all(|c| c.is_ascii_uppercase()) || !path.contains(&node.to_string()) {
    for neighbour in cave.get(&node).unwrap() {
      traverse(cave, neighbour, new_path.clone(), routes, twice);
    }
  } else if twice.is_none() && node != "start" {
    for neighbour in cave.get(&node).unwrap() {
      traverse(cave, neighbour, new_path.clone(), routes, Some(node));
    }
  }
}

#[aoc(day12, part1)]
pub fn all_paths(input: &str) -> u32 {
  let cave: HashMap<&str, HashSet<&str>> = map_cave(input);
  let mut routes: Vec<Vec<String>> = Vec::new();
  traverse(&cave, "start", Vec::new(), &mut routes, Some(""));
  routes.len() as u32
}

#[aoc(day12, part2)]
pub fn all_paths_more_time(input: &str) -> u32 {
  let cave: HashMap<&str, HashSet<&str>> = map_cave(input);
  let mut routes: Vec<Vec<String>> = Vec::new();
  traverse(&cave, "start", Vec::new(), &mut routes, None);
  routes.len() as u32
}
