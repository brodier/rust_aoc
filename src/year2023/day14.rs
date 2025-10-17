use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
    tilters: [Tilter;4]
}

impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        let grid = input.to_string();
        let tilters = [Tilter::build(&grid, &Orient::NORTH),
            Tilter::build(&grid, &Orient::WEST),
            Tilter::build(&grid, &Orient::SOUTH),
            Tilter::build(&grid, &Orient::EAST)];
        Puzzle { step, input, tilters }
    }

    fn solve(&self) -> String {
        // On init_rows rows are west oriented for tilt and correctly oriented for compute_north_load
        let mut known_pos = HashMap::new();
        if self.step == 1 {

            let north_tilted_grid = self.tilters[0].tilt_grid(self.input);
            return get_north_load(&north_tilted_grid).to_string();
        }
        let mut grid = self.input.to_string();
        known_pos.insert(grid.clone(),0);
        let mut loop_start = 0;
        let mut loop_len = 0;
        for i in 0..1_000_000_000 {
            grid = self.spin_cycle(grid);
            let curr_spin_number = i + 1;
            if let Some(old_pos) = known_pos.insert(grid.clone(), curr_spin_number) {
                loop_start = curr_spin_number;
                loop_len = curr_spin_number - old_pos;
                break;
            }
        }
        let n = (1_000_000_000 - loop_start)%loop_len;
        for _ in 0..n {
            grid = self.spin_cycle(grid);
        }
        return get_north_load(&grid).to_string();
    }

    fn spin_cycle(&self, grid:String) -> String {
        let mut grid = grid.to_string();
        for tilter in self.tilters.iter() {
            grid = tilter.tilt_grid(&grid);
        }
        grid
    }

}

fn get_north_load(grid:&String) -> usize {
    let mut total_load = 0;
    let mut row_load = grid.lines().count();
    for line in grid.lines() {
        let l_bytes = line.as_bytes();
        for &b in l_bytes.iter() {
            match b {
                b'O' => total_load += row_load,
                _ => {}
            }
        }
        row_load -=1;
    }
    total_load
}


enum Orient {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

#[derive(Debug)]
struct Tilter {
    total_length: usize,
    nb_rows: usize,
    row_len: usize,
    initial_pos: usize,
    cell_step: usize,
    row_step: usize
}

impl Tilter {
    fn build(grid:&String, orient:&Orient) -> Tilter {
        let total_length = grid.len();
        let line_len = grid.find("\n").unwrap();
        let nb_lines = grid.lines().count();
        let (nb_rows, row_len, initial_pos, cell_step, row_step) = match orient {
            Orient::NORTH => (line_len, nb_lines,0,line_len+1,0),
            Orient::WEST => (nb_lines, line_len,0,1,1),
            Orient::SOUTH => (line_len, nb_lines,total_length-line_len,total_length-line_len-1,total_length+2),
            Orient::EAST => (nb_lines, line_len,total_length-1,total_length-1,total_length-1),
        };
        Tilter{total_length, nb_rows, row_len, initial_pos, cell_step, row_step}
    } 

    fn tilt_grid(&self, grid:&str) -> String {
        let mut rows = grid.as_bytes().to_owned();
        let mut i = self.initial_pos;
        for _ in 0..self.nb_rows {
            let mut next_swap_index = i;
            for _ in 0..self.row_len {
                match rows[i] {
                    b'O' => {
                        if next_swap_index!=i {
                            rows[i]=b'.';
                            rows[next_swap_index]=b'O';
                        }
                        next_swap_index = (next_swap_index + self.cell_step) % self.total_length;
                    },
                    b'#' => next_swap_index = (i + self.cell_step) % self.total_length,
                    _ => {}
                }
                i = (i + self.cell_step) % self.total_length;
            }
            i = (i + self.row_step) % self.total_length;
        }
        String::from_utf8(rows).unwrap()
    }

}


pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
