use derive_more::derive::{Add, Sub};

#[derive(Debug, Copy, Clone, Add, Sub, Hash, PartialEq, Eq)]
pub struct Coord(pub isize, pub isize);

pub struct BoundingBox {
    pub xmin: isize,
    pub xmax: isize,
    pub ymin: isize,
    pub ymax: isize,
}

impl BoundingBox {
    pub fn contains(&self, coord: &Coord) -> bool {
        (self.xmin..self.xmax).contains(&coord.0) && (self.ymin..self.ymax).contains(&coord.1)
    }

    pub fn inside_coords(&self, coord: &Coord) -> Option<(usize, usize)> {
        self.contains(coord).then_some((
            (coord.0 - self.xmin) as usize,
            (coord.1 - self.ymin) as usize,
        ))
    }
    pub fn try_from_size(w: usize, h: usize) -> Option<Self> {
        Some(BoundingBox {
            xmin: 0,
            xmax: w.try_into().ok()?,
            ymin: 0,
            ymax: h.try_into().ok()?,
        })
    }
}

pub trait Field {
    type Out;
    #[allow(dead_code)]
    fn get(&self, coords: &Coord) -> Option<&Self::Out>;
    #[allow(dead_code)]
    fn get_mut(&mut self, coords: &Coord) -> Option<&mut Self::Out>;
    fn definition_area(&self) -> &BoundingBox;
}

use huparse::table::Table;
struct TableField<T> {
    bounding_box: BoundingBox,
    values: Table<T>,
}

enum TableFieldError {
    TableTooBig,
}

impl<T> TryFrom<Table<T>> for TableField<T> {
    type Error = TableFieldError;

    fn try_from(value: Table<T>) -> Result<Self, Self::Error> {
        use TableFieldError::*;
        let valid_width: isize = value.width().try_into().map_err(|_| TableTooBig)?;
        let valid_height: isize = value.height().try_into().map_err(|_| TableTooBig)?;
        Ok(TableField {
            values: value,
            bounding_box: BoundingBox {
                xmin: 0,
                xmax: valid_width,
                ymin: 0,
                ymax: valid_height,
            },
        })
    }
}

impl<T> Field for TableField<T> {
    type Out = T;

    fn get(&self, coords: &Coord) -> Option<&Self::Out> {
        self.definition_area()
            .inside_coords(coords)
            .and_then(|(x, y)| self.values.get(y, x))
    }

    fn get_mut(&mut self, coords: &Coord) -> Option<&mut Self::Out> {
        self.definition_area()
            .inside_coords(coords)
            .and_then(|(x, y)| self.values.get_mut(y, x))
    }

    fn definition_area(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Ri,
    Do,
    Le,
}

pub fn moved(pos: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => Coord(pos.0, pos.1 - 1),
        Direction::Ri => Coord(pos.0 + 1, pos.1),
        Direction::Do => Coord(pos.0, pos.1 + 1),
        Direction::Le => Coord(pos.0 - 1, pos.1),
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
