use crate::utils::grid::GridError;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}

#[derive(Debug,Clone)]
enum Orient {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

#[derive(Debug,Clone)]
struct Grid<'a> {
    size:(usize,usize),
    input:&'a str,
}

impl Grid<'_> {
    
    fn build<'a>(input:&'a str) -> Grid<'a> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Grid{size:(width,height), input}
    }

    fn up(&self, pos:(usize,usize)) -> Result<(usize,usize), GridError> {
        if pos.1 < 1 {
            return Err(GridError::OutOfBoard);
        } else {
            return Ok((pos.0,pos.1-1));
        }        
    }

    fn left(&self, pos:(usize,usize)) -> Result<(usize,usize), GridError> {
        if pos.0 < 1 {
            return Err(GridError::OutOfBoard);
        } else {
            return Ok((pos.0-1,pos.1));
        }        
    }

    fn down(&self, pos:(usize,usize)) -> Result<(usize,usize), GridError> {
        if pos.1 + 1 == self.size.1 {
            return Err(GridError::OutOfBoard);
        } else {
            return Ok((pos.0,pos.1+1));
        }        
    }

    fn right(&self, pos:(usize,usize)) -> Result<(usize,usize), GridError> {
        if pos.0 + 1 == self.size.0 {
            return Err(GridError::OutOfBoard);
        } else {
            return Ok((pos.0+1,pos.1));
        }        
    }
    
    fn get(&self, pos:(usize,usize)) -> u8  {
        self.input.as_bytes()[pos.0 + pos.1 * (self.size.0 + 1)]
    }

}

#[derive(Debug)]
struct Animal {
    init_pos:(usize,usize),
    pos:(usize,usize),
    orient:Orient
}


impl Animal {

    fn build(grid:&Grid) -> Animal {
        let i = grid.input.find("S").unwrap();
        // println!("S is at {}", i);
        let init_pos = (i%(grid.size.0+1),i/(grid.size.0+1));
        let mut animal = Animal{init_pos, pos:init_pos.clone(), orient:Orient::UP};
        // println!("Start from {:?} Grid size {:?}", animal.init_pos, grid.size);
        animal.init_orient(grid);
        animal 
    }

    fn init_orient(&mut self, grid:&Grid) {
        for o in [Orient::UP, Orient::LEFT,Orient::DOWN,Orient::RIGHT] {
            self.orient = o;
            if self.walk(grid).is_ok() {
                self.pos = self.init_pos;
                return;
            }
            self.pos = self.init_pos;
        }
        panic!("Not any valid start orientation");   
    }

    fn walk(&mut self, grid:&Grid) -> Result<Orient,GridError> {
        let new_pos = match self.orient {
            Orient::UP => grid.up(self.pos),
            Orient::LEFT => grid.left(self.pos),
            Orient::DOWN => grid.down(self.pos),
            Orient::RIGHT => grid.right(self.pos),
        };
        if new_pos.is_err() {
            eprintln!("New pos is out of board");
            return Err(GridError::OutOfBoard);
        } 
        self.pos = new_pos.unwrap();
        let new_orient = self.update_orient(grid);
        if new_orient.is_err() {
            if grid.get(self.pos) != b'S' {
                eprintln!("New pos {:?} not compatible with orient", self.pos);
            }
        }  
        return new_orient;
    }

    fn update_orient(&mut self, grid:&Grid) -> Result<Orient,GridError> {
        match self.orient {
            Orient::UP => {
                match grid.get(self.pos) {
                    b'|' => Ok(Orient::UP),
                    b'7' => Ok(Orient::LEFT),
                    b'F' => Ok(Orient::RIGHT),
                    _ => Err(GridError::Wall)
                }
            },
            Orient::LEFT => {
                match grid.get(self.pos) {
                    b'-' => Ok(Orient::LEFT),
                    b'F' => Ok(Orient::DOWN),
                    b'L' => Ok(Orient::UP),
                    _ => Err(GridError::Wall)
                }
            },
            Orient::DOWN => {
                match grid.get(self.pos) {
                    b'J' => Ok(Orient::LEFT),
                    b'L' => Ok(Orient::RIGHT),
                    b'|' => Ok(Orient::DOWN),
                    _ => Err(GridError::Wall)
                }
            },
            Orient::RIGHT => {
                match grid.get(self.pos) {
                    b'-' => Ok(Orient::RIGHT),
                    b'7' => Ok(Orient::DOWN),
                    b'J' => Ok(Orient::UP),
                    _ => Err(GridError::Wall)
                }
            }
        }
    }
}

impl Puzzle<'_> {
    fn build<'a>(step: usize, input: &'a str) -> Puzzle<'a> {
        Puzzle{ step , input }
    }

    fn solve(&self) -> String {
        let grid = Grid::build(self.input);
        let mut animal = Animal::build(&grid);
        let mut counter = 1;
        // println!("ready to walk with animal: {:?}", animal);
        let mut points = Vec::new();
        points.push(animal.init_pos);
        while let Ok(new_orient) = animal.walk(&grid) {
            match grid.get(animal.pos) {
                b'7' | b'J' | b'F' | b'L' => points.push(animal.pos),
                _ => {}
            }
            animal.orient = new_orient;
            counter += 1;
        }
        counter /= 2;
        // println!("Computed counter : {}", counter);
        if self.step == 1 {
            counter.to_string()
        } else {
            (self.get_nb_interior_tiles(points) + 1 - counter).to_string()
        }
    }

    fn get_nb_interior_tiles(&self, points:Vec<(usize,usize)>) -> usize {
        let a = points.last().unwrap();
        let mut a = (a.0 as i64, a.1 as i64);
        let mut sum:i64 = 0;
        for itt in points.iter() {
            let b:(i64,i64) = (itt.0 as i64, itt.1 as i64);
            sum += a.1 * b.0 - a.0 * b.1;
            a = b;
        }
        sum /= 2;
        // println!("Computed area : {}", sum);
        sum.abs() as usize
    }
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    // println!("Puzzle:\n{}", p.input);
    p.solve()
}
