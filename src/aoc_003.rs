use std::collections::{HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(center_intersect_distance(parse("R8,U5,L5,D3"), parse("U7,R6,D4,L4")), 6);
    }
}

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn cmd_steps(&mut self, cmd: &Cmd) {
        let step = match cmd.direction {
            Dir::Up(a) => Pos { x: 0, y: -1 },
            Dir::Down(a) => Pos { x: 0, y: 1 },
            Dir::Left(a) => Pos { x: -1, y: 0 },
            Dir::Right(a) => Pos { x: 1, y: 0 },
        };
        for i in 0..cmd.distance {
            self += step;
        }
    }
}

impl std::ops::AddAssign for Pos{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum Dir {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

struct Cmd {
    direction: Dir,
    distance: i32,
}

fn parse(wire: &str) -> Vec<Cmd> {
    wire.split(",").map(|cmd: &str| {
        let num = (cmd[1..]).parse::<u32>().unwrap();
        match cmd[0] {
            'U' => Cmd::Up(num),
            'D' => Cmd::Down(num),
            'L' => Cmd::Left(num),
            'R' => Cmd::Right(num),
            _ => panic!("Unknown cmd: {}", cmd),
        }
    }).collect()
}

fn center_intersect_distance(wire1: Vec<Cmd>, wire2: Vec<Cmd>) -> i32 {
    let mut set = HashSet::new();
}

fn fill(wire: Vec<Cmd>) -> HashSet<Pos> {
    let mut pos = Pos::origin();
    for line in wire {}

    ()
}