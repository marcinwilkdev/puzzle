//! [Direction] used with [Move](super::Move)

/// Direction of [Move]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Gives coordinate `(row, column)` difference for performing move in direction.
    pub fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    /// Return opposite direction.
    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions_match() {
        let mut sum_directions_movements = (0, 0);

        sum_directions_movements.0 += Direction::Up.as_coordinates().0;
        sum_directions_movements.1 += Direction::Up.as_coordinates().1;

        sum_directions_movements.0 += Direction::Down.as_coordinates().0;
        sum_directions_movements.1 += Direction::Down.as_coordinates().1;

        sum_directions_movements.0 += Direction::Left.as_coordinates().0;
        sum_directions_movements.1 += Direction::Left.as_coordinates().1;

        sum_directions_movements.0 += Direction::Right.as_coordinates().0;
        sum_directions_movements.1 += Direction::Right.as_coordinates().1;

        assert_eq!((0, 0), sum_directions_movements);
    }

    #[test]
    fn opposite_directions_work() {
        let vertical_directions = Direction::Up;

        assert_eq!(Direction::Down, vertical_directions.opposite());
        assert_eq!(Direction::Up, vertical_directions.opposite().opposite());

        let horizontal_directions = Direction::Left;

        assert_eq!(Direction::Right, horizontal_directions.opposite());
        assert_eq!(Direction::Left, horizontal_directions.opposite().opposite());
    }
}
