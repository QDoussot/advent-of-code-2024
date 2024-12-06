pub type Coord = (isize, isize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Ri,
    Do,
    Le,
}

pub fn moved(pos: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Ri => (pos.0 + 1, pos.1),
        Direction::Do => (pos.0, pos.1 + 1),
        Direction::Le => (pos.0 - 1, pos.1),
    }
}

impl Direction {
    pub fn rotate(self) -> Direction {
        match self {
            Direction::Up => Direction::Ri,
            Direction::Ri => Direction::Do,
            Direction::Do => Direction::Le,
            Direction::Le => Direction::Up,
        }
    }
}

