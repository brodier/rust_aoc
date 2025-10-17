use std::{cell, collections::HashMap};

#[derive(Debug)]
struct Puzzle<'a> {
    step:usize, 
    input: &'a str,
}



impl Puzzle<'_> {
    fn build<'a>(step:usize, input:&'a str) -> Puzzle<'a> {
        Puzzle { step, input }
    }

    fn solve(&self) -> String {
        // On init_rows rows are west oriented for tilt and correctly oriented for compute_north_load
        let mut known_pos = HashMap::new();
        let mut rows = init_rows(self.input);  
        if self.step == 1 {
            return compute_north_load(&rows).to_string();
        }
        let grid = to_grid(&rows);
        known_pos.insert(grid,0);
        let mut loop_start = 0;
        let mut loop_len = 0;
        for i in 0..1_000_000_000 {
            rows = spin_cycle(&rows);
            let curr_spin_number = i + 1;
            if let Some(old_pos) = known_pos.insert(to_grid(&rows), curr_spin_number) {
                loop_start = curr_spin_number;
                loop_len = curr_spin_number - old_pos;
                break;
            }
        }
        let n = (1_000_000_000 - loop_start)%loop_len;
        for _ in 0..n {
            rows = spin_cycle(&rows);
        }
        return compute_north_load_without_tilt(&rows).to_string();
    }

}

fn spin_cycle(rows:&Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rows = rotate_rows(&rotate_rows(&rotate_rows(rows)));
    rows = rotate_rows(&tilt_rows(&rows)); 
    rows = rotate_rows(&tilt_rows(&rows)); 
    rows = rotate_rows(&tilt_rows(&rows)); 
    rows = rotate_rows(&tilt_rows(&rows)); 
    rotate_rows(&rows)
}

fn to_grid(rows:&Vec<Vec<u8>>) -> String {
    rows.iter().map(|r| String::from_utf8(r.clone()).unwrap()).collect::<Vec<String>>().join("\n")
}


fn compute_north_load_without_tilt(rows:&Vec<Vec<u8>>) -> usize {
    let mut total_load = 0;
    let mut load = rows.len();

    for line in rows.iter() {
        for (_i,&b) in line.iter().enumerate() {
            match b {
                b'O' => total_load += load,
                _ => {}
            }
        }
        load -=1;
    }
    total_load
}

fn compute_north_load(rows:&Vec<Vec<u8>>) -> usize {
    let mut total_load = 0;
    let mut offsets:Vec<usize> = vec![0;rows[0].len()];
    let mut load = rows.len();

    for line in rows.iter() {
        for (i,&b) in line.iter().enumerate() {
            match b {
                b'#' => offsets[i] = 0,
                b'.' => offsets[i] += 1,
                b'O' => total_load += load + offsets[i],
                _ => unreachable!()
            }
        }
        load -=1;
    }
    total_load
}

fn init_rows(input:&str) -> Vec<Vec<u8>> {
    input.split("\n").map(|line| line.as_bytes().to_owned()).collect()
}

// Step 2 need fews functions for computing tilt 
fn rotate_rows(rows:&Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows_len = rows[0].len();
    let nb_rows = rows.len();
    let mut grid:Vec<Vec<u8>> = vec![vec![b'.';nb_rows];rows_len];
    for i in 0..rows_len{
        for j in 0..nb_rows {
            grid[i][j] = rows[nb_rows -1 - j][i];
        }
    }
    grid
}

fn tilt_rows(rows:&Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    rows.iter().map(|row| tilt_row(row)).collect()
}

enum Orient {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

fn _tilt_grid(grid:&String, orient:&Orient) -> Vec<u8> {
    let total_length = grid.len();
    let line_len = grid.find("\n").unwrap();
    let nb_lines = grid.lines().count();
    let (nb_rows, row_len, initial_pos, cell_step, row_step) = match orient {
        Orient::NORTH => (line_len, nb_lines,0,line_len+1,line_len+1),
        Orient::WEST => (nb_lines, line_len,0,1,1),
        Orient::SOUTH => (line_len, nb_lines,0,1,1),
        Orient::EAST => (nb_lines, line_len,total_length-1,total_length-1,total_length-1),
    };
    let mut rows = grid.as_bytes().to_owned();
    let mut i = initial_pos;
    for _ in 0..nb_rows {
        if i != initial_pos {
            i = (i + row_step) % total_length;
        }
        let mut next_swap_index = i;
        for _ in 0..row_len {
            match rows[i] {
                b'O' => {
                    if next_swap_index!=i {
                        rows[i]=b'.';
                        rows[next_swap_index]=b'O';
                    }
                    next_swap_index = (next_swap_index + cell_step) % total_length;
                },
                b'#' => next_swap_index = (i + cell_step) % total_length,
                _ => {}
            }
            i = (i + cell_step) % total_length;
        }
    }
    rows
}

fn tilt_row(line:&Vec<u8>) -> Vec<u8> {
    let mut row = line.clone();
    let mut next_swap_index = 0;
    for i in 0..row.len() {
        match row[i] {
            b'O' => {
                if next_swap_index<i {
                    row[i]=b'.';
                    row[next_swap_index]=b'O';
                }
                next_swap_index += 1;
            },
            b'#' => next_swap_index = i + 1,
            _ => {}
        }
    }
    row
}

pub fn solve(step: usize, input: String) -> String {
    let p = Puzzle::build(step, &input);
    p.solve()
}
