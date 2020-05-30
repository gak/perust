use std::collections::{HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(center_intersect_distance(
            parse("R8,U5,L5,D3"),
            parse("U7,R6,D4,L4"),
        ), 6);
        assert_eq!(center_intersect_distance(
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse("U62,R66,U55,R34,D71,R55,D58,R83"),
        ), 159);
        assert_eq!(center_intersect_distance(
            parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ), 135);
    }

    #[test]
    fn big() {
        let inp: Vec<&str> = include_str!("aoc_003_input.txt")
            .trim()
            .split_ascii_whitespace()
            .collect();
        assert_eq!(center_intersect_distance(
            parse(inp[0]),
            parse(inp[1]),
        ), 248);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn cmd_steps(&mut self, cmd: &Cmd) -> Vec<Pos> {
        let mut steps = vec![];
        let step = match cmd.direction {
            Dir::Up => Pos { x: 0, y: -1 },
            Dir::Down => Pos { x: 0, y: 1 },
            Dir::Left => Pos { x: -1, y: 0 },
            Dir::Right => Pos { x: 1, y: 0 },
        };
        for _ in 0..cmd.distance {
            *self += step;
            steps.push(*self);
        }
        steps
    }

    fn man_dist_to_origin(self: &Self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Cmd {
    direction: Dir,
    distance: i32,
}

fn parse(wire: &str) -> Vec<Cmd> {
    wire.split(",").map(|cmd: &str| {
        let c: Vec<char> = cmd.chars().collect();
        let distance = (c[1..]).iter().collect::<String>().parse::<u32>().unwrap() as i32;
        let direction = match c[0] {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Unknown cmd: {}", cmd),
        };
        Cmd {
            direction,
            distance,
        }
    }).collect()
}

fn center_intersect_distance(a: Vec<Cmd>, b: Vec<Cmd>) -> i32 {
    let a = fill(a);
    let b = fill(b);
    let intersect = a.intersection(&b);
    let mut closest = None;
    for cross in intersect {
        if closest.is_none() || cross.man_dist_to_origin() < closest.unwrap() {
            closest = Some(cross.man_dist_to_origin())
        }
    }
    closest.unwrap()
}

fn fill(wire: Vec<Cmd>) -> HashSet<Pos> {
    let mut used = HashSet::new();
    let mut pos = Pos::origin();
    for line in wire {
        for step in pos.cmd_steps(&line) {
            used.insert(step);
        }
    }
    used
}