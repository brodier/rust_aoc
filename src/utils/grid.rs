

#[derive(Debug,Clone,Copy)]
pub enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(Debug)]
pub enum GridError {
    OutOfBoard,
    Wall
}

impl Dir {

    pub fn get_next(&self, pos:(usize,usize), board_size:(usize,usize)) -> Result<(usize,usize), GridError> {
        match self {
            Dir::UP => if pos.1 > 0 {
                 Ok((pos.0, pos.1 - 1))
            } else {
                Err(GridError::OutOfBoard)
            },
            Dir::DOWN => if pos.1 + 1 < board_size.1 {
                Ok((pos.0,pos.1+1))
            } else  {
                Err(GridError::OutOfBoard)
            },
            Dir::LEFT => if pos.0 > 0 {
                Ok((pos.0-1,pos.1))
            } else {
                Err(GridError::OutOfBoard)
            },
            Dir::RIGHT => if pos.0 +1 < board_size.0 {
                Ok((pos.0+1,pos.1))
            } else {
                Err(GridError::OutOfBoard)
            }
        }
    }

    pub fn left(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::RIGHT,
            Dir::RIGHT => Dir::UP,
            Dir::UP => Dir::LEFT,
            Dir::LEFT => Dir::DOWN,
        }
    }

    pub fn right(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
        }
    }

    pub fn back(&self) -> Dir {
        match self {
            Dir::DOWN => Dir::UP,
            Dir::LEFT => Dir::RIGHT,
            Dir::UP => Dir::DOWN,
            Dir::RIGHT => Dir::LEFT,
        }
    }

    pub fn all() -> [Dir;4] {
        [Dir::UP, Dir::LEFT, Dir::DOWN, Dir::RIGHT]
    }
}